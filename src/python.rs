use std::borrow::Cow;
use std::fs::File;
use std::io::Write;
use std::sync::RwLock;

use pyo3::exceptions::PyValueError;
use pyo3::types::{PyBytes, PySequence};
use pyo3::{prelude::*, types::PyDict};
use reqwest::header::{HeaderName, HeaderValue};
use tokio::runtime::Runtime;

use crate::audio::play_owned_audio_blocking;
use crate::get_rest_endpoint_by_region;
use crate::parse::{parse_pitch, parse_rate, parse_style_degree};
use crate::synthesizer::UnifiedSynthesizer;
use crate::{
    AudioFormat, AuthOptions, TextOptions, get_default_voice_by_locale,
    get_websocket_endpoint_by_region, synthesizer::SynthesizerConfig,
};

#[pymodule]
fn aspeak(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    #[cfg(debug_assertions)]
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();
    crate::types::register_python_items(py, m)?;
    crate::audio::register_python_items(py, m)?;
    m.add_class::<SpeechService>()?;
    Ok(())
}

#[pyclass]
struct SpeechService {
    synthesizer: RwLock<Box<dyn UnifiedSynthesizer + Sync>>,
    runtime: Runtime,
}

impl SpeechService {
    fn parse_text_options<'a>(
        options: Option<&'a Bound<PyDict>>,
    ) -> PyResult<Option<TextOptions<'a>>> {
        options
            .map(|opts| {
                Ok::<TextOptions, PyErr>(TextOptions {
                    pitch: opts
                        .get_item("pitch")?
                        .as_ref()
                        .map(|p| p.extract())
                        .transpose()?
                        .map(parse_pitch)
                        .transpose()?,
                    rate: opts
                        .get_item("rate")?
                        .as_ref()
                        .map(|r| r.extract())
                        .transpose()?
                        .map(parse_rate)
                        .transpose()?,
                    voice: {
                        if let Some(voice) = opts
                            .get_item("voice")?
                            .as_ref()
                            .map(|p| p.extract::<&str>())
                            .transpose()?
                        {
                            Cow::Owned(voice.to_string())
                        } else {
                            let v = opts.get_item("locale")?;
                            let locale = v
                                .as_ref()
                                .map(|v| v.extract())
                                .transpose()?
                                .unwrap_or("en-US");
                            Cow::Borrowed(get_default_voice_by_locale(locale).ok_or_else(|| {
                                PyValueError::new_err(format!(
                                    "No default voice for locale: {}",
                                    locale
                                ))
                            })?)
                        }
                    },
                    rich_ssml_options: {
                        let style = opts
                            .get_item("style")?
                            .as_ref()
                            .map(|s| s.extract())
                            .transpose()?
                            .map(|s: &str| s.to_string().into());
                        let style_degree = opts
                            .get_item("style_degree")?
                            .as_ref()
                            .map(|l| l.extract())
                            .transpose()?
                            .map(parse_style_degree)
                            .transpose()?;
                        let role = opts.get_item("role")?.map(|r| r.extract()).transpose()?;
                        if style.is_some() || style_degree.is_some() || role.is_some() {
                            Some(crate::types::RichSsmlOptions {
                                style,
                                style_degree,
                                role,
                            })
                        } else {
                            None
                        }
                    },
                })
            })
            .transpose()
    }
}

#[pymethods]
impl SpeechService {
    #[new]
    #[pyo3(signature = (audio_format = AudioFormat::Riff24Khz16BitMonoPcm, **options))]
    fn new(audio_format: AudioFormat, options: Option<&Bound<PyDict>>) -> PyResult<Self> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()?;

        let mode = options
            .map(|dict| dict.get_item("mode"))
            .transpose()?
            .flatten();
        let mode = mode
            .as_ref()
            .map(|e| e.extract::<&str>())
            .transpose()?
            .unwrap_or("rest");

        if mode != "rest" && mode != "websocket" {
            return Err(PyValueError::new_err(format!(
                "Invalid synthesizer mode: {}",
                mode
            )));
        }

        let endpoint = options
            .map(|dict| dict.get_item("endpoint"))
            .transpose()?
            .flatten();
        let endpoint =
            if let Some(endpoint) = endpoint.as_ref().map(|e| e.extract::<&str>()).transpose()? {
                Cow::Borrowed(endpoint)
            } else {
                options
                    .map(|dict| dict.get_item("region"))
                    .transpose()?
                    .flatten()
                    .as_ref()
                    .map(|e| e.extract::<&str>())
                    .transpose()?
                    .map(|r| match mode {
                        "rest" => get_rest_endpoint_by_region(r),
                        "websocket" => get_websocket_endpoint_by_region(r),
                        _ => unreachable!(),
                    })
                    .map(Cow::Owned)
                    .ok_or_else(|| {
                        PyValueError::new_err("No endpoint or region is specified!".to_string())
                    })?
            };
        let key: Option<String> = options
            .map(|dict| dict.get_item("key"))
            .transpose()?
            .flatten()
            .map(|k| k.extract())
            .transpose()?;
        let token: Option<String> = options
            .map(|dict| dict.get_item("token"))
            .transpose()?
            .flatten()
            .map(|k| k.extract())
            .transpose()?;
        let proxy: Option<String> = options
            .map(|dict| dict.get_item("proxy"))
            .transpose()?
            .flatten()
            .map(|p| p.extract())
            .transpose()?;
        let headers = options
            .map(|dict| dict.get_item("headers"))
            .transpose()?
            .flatten();
        let headers = headers
            .as_ref()
            .map(|h| h.downcast::<PySequence>())
            .transpose()?;
        let headers = if let Some(headers) = headers {
            headers
                .try_iter()?
                .map(|header| {
                    header.and_then(|header| {
                        let header = header.downcast::<PySequence>()?;
                        let name = header.get_item(0)?;
                        let name = name.as_ref().extract::<&str>()?;
                        let value = header.get_item(1)?;
                        let value = value.as_ref().extract::<&str>()?;
                        Ok((
                            HeaderName::from_bytes(name.as_bytes()).map_err(|e| {
                                PyValueError::new_err(format!("Invalid header name: {e}"))
                            })?,
                            HeaderValue::from_str(value).map_err(|e| {
                                PyValueError::new_err(format!("Invalid header value: {e}"))
                            })?,
                        ))
                    })
                })
                .collect::<PyResult<Vec<_>>>()?
        } else {
            Vec::new()
        };
        Ok(Self {
            synthesizer: RwLock::new(runtime.block_on(async {
                let conf = SynthesizerConfig::new(
                    AuthOptions {
                        endpoint: Cow::Borrowed(&endpoint),
                        key: key.as_deref().map(Cow::Borrowed),
                        headers: Cow::Borrowed(headers.as_slice()),
                        token: token.as_deref().map(Cow::Borrowed),
                        proxy: proxy.as_deref().map(Cow::Borrowed),
                    },
                    audio_format,
                );
                let boxed: Box<dyn UnifiedSynthesizer + Sync> = match mode {
                    "rest" => Box::new(conf.rest_synthesizer()?),
                    "websocket" => Box::new(conf.connect_websocket().await?),
                    _ => unreachable!(),
                };
                Ok::<Box<dyn UnifiedSynthesizer + Sync>, PyErr>(boxed)
            })?),
            runtime,
        })
    }

    fn speak_ssml(&self, ssml: &str) -> PyResult<()> {
        let buffer = self.runtime.block_on(
            self.synthesizer
                .write()
                .unwrap()
                .as_mut()
                .process_ssml(ssml),
        )?;
        play_owned_audio_blocking(buffer)?;
        Ok(())
    }

    #[pyo3(signature = (ssml, **options))]
    fn synthesize_ssml<'a>(
        &self,
        ssml: &str,
        options: Option<Bound<PyDict>>,
        py: Python<'a>,
    ) -> PyResult<Option<Bound<'a, PyBytes>>> {
        let data = self.runtime.block_on(
            self.synthesizer
                .write()
                .unwrap()
                .as_mut()
                .process_ssml(ssml),
        )?;
        if let Some(output) = options
            .map(|d| d.get_item("output"))
            .transpose()?
            .flatten()
            .as_ref()
            .map(|f| f.extract::<&str>())
            .transpose()?
        {
            let mut file = File::create(output)?;
            file.write_all(&data)?;
            Ok(None)
        } else {
            Ok(Some(PyBytes::new(py, &data)))
        }
    }

    #[pyo3(signature = (text, **options))]
    fn speak_text(&self, text: &str, options: Option<Bound<PyDict>>) -> PyResult<()> {
        let buffer =
            self.runtime
                .block_on(self.synthesizer.write().unwrap().as_mut().process_text(
                    text,
                    &Self::parse_text_options(options.as_ref())?.unwrap_or_default(),
                ))?;
        play_owned_audio_blocking(buffer)?;
        Ok(())
    }

    #[pyo3(signature = (text, **options))]
    fn synthesize_text<'a>(
        &self,
        text: &str,
        options: Option<Bound<PyDict>>,
        py: Python<'a>,
    ) -> PyResult<Option<Bound<'a, PyBytes>>> {
        let data =
            self.runtime
                .block_on(self.synthesizer.write().unwrap().as_mut().process_text(
                    text,
                    &Self::parse_text_options(options.as_ref())?.unwrap_or_default(),
                ))?;
        if let Some(output) = options
            .map(|d| d.get_item("output"))
            .transpose()?
            .flatten()
            .as_ref()
            .map(|f| f.extract::<&str>())
            .transpose()?
        {
            let mut file = File::create(output)?;
            file.write_all(&data)?;
            Ok(None)
        } else {
            Ok(Some(PyBytes::new(py, &data)))
        }
    }
}
