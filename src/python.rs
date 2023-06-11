use std::borrow::Cow;
use std::cell::RefCell;
use std::fs::File;
use std::io::Write;

use pyo3::exceptions::{PyOSError, PyValueError};
use pyo3::types::{PyBytes, PySequence};
use pyo3::{prelude::*, types::PyDict};
use reqwest::header::{HeaderName, HeaderValue};
use tokio::runtime::Runtime;

use crate::audio::play_owned_audio_blocking;
use crate::constants::DEFAULT_ENDPOINT;
use crate::parse::{parse_pitch, parse_rate, parse_style_degree};
use crate::{
    get_default_voice_by_locale, get_endpoint_by_region,
    synthesizer::{SynthesizerConfig, WebsocketSynthesizer},
    AudioFormat, AuthOptions, TextOptions,
};

#[pymodule]
fn aspeak(py: Python, m: &PyModule) -> PyResult<()> {
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
    audio_format: AudioFormat,
    endpoint: String,
    key: Option<String>,
    auth_token: Option<String>,
    proxy: Option<String>,
    headers: Vec<(HeaderName, HeaderValue)>,
    synthesizer: RefCell<Option<WebsocketSynthesizer>>,
    runtime: Runtime,
}

impl SpeechService {
    fn parse_text_options(options: Option<&PyDict>) -> PyResult<Option<TextOptions>> {
        options
            .map(|opts| {
                Ok::<TextOptions, PyErr>(TextOptions {
                    pitch: opts
                        .get_item("pitch")
                        .map(|p| p.extract())
                        .transpose()?
                        .map(parse_pitch)
                        .transpose()?,
                    rate: opts
                        .get_item("rate")
                        .map(|r| r.extract())
                        .transpose()?
                        .map(parse_rate)
                        .transpose()?,
                    voice: {
                        if let Some(voice) =
                            opts.get_item("voice").map(|p| p.extract()).transpose()?
                        {
                            Cow::Borrowed(voice)
                        } else {
                            let locale = opts
                                .get_item("locale")
                                .map(|l| l.extract())
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
                            .get_item("style")
                            .map(|s| s.extract())
                            .transpose()?
                            .map(Cow::Borrowed);
                        let style_degree = opts
                            .get_item("style_degree")
                            .map(|l| l.extract())
                            .transpose()?
                            .map(parse_style_degree)
                            .transpose()?;
                        let role = opts.get_item("role").map(|r| r.extract()).transpose()?;
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
    fn new(audio_format: AudioFormat, options: Option<&PyDict>) -> PyResult<Self> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()?;
        let endpoint = if let Some(endpoint) = options
            .and_then(|dict| dict.get_item("endpoint"))
            .map(|e| e.extract::<&str>())
            .transpose()?
        {
            Cow::Borrowed(endpoint)
        } else {
            options
                .and_then(|dict| dict.get_item("region"))
                .map(|e| e.extract::<&str>())
                .transpose()?
                .map(get_endpoint_by_region)
                .map(Cow::Owned)
                .or_else(|| DEFAULT_ENDPOINT.map(Cow::Borrowed))
                .ok_or_else(|| PyValueError::new_err("No endpoint is specified!".to_string()))?
        };
        let key: Option<String> = options
            .and_then(|dict| dict.get_item("key"))
            .map(|k| k.extract())
            .transpose()?;
        let token: Option<String> = options
            .and_then(|dict| dict.get_item("token"))
            .map(|k| k.extract())
            .transpose()?;
        let proxy: Option<String> = options
            .and_then(|dict| dict.get_item("proxy"))
            .map(|p| p.extract())
            .transpose()?;
        let headers = options
            .and_then(|dict| dict.get_item("headers"))
            .map(|h| h.downcast::<PySequence>())
            .transpose()?;
        let headers = if let Some(headers) = headers {
            headers
                .iter()?
                .map(|header| {
                    header.and_then(|header| {
                        let header = header.downcast::<PySequence>()?;
                        let name = header.get_item(0)?.extract::<&str>()?;
                        let value = header.get_item(1)?.extract::<&str>()?;
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
            audio_format,
            endpoint: endpoint.into_owned(),
            key,
            auth_token: token,
            headers,
            proxy,
            synthesizer: RefCell::new(None),
            runtime,
        })
    }

    fn connect(&self) -> PyResult<()> {
        self.synthesizer.borrow_mut().replace(
            self.runtime.block_on(
                SynthesizerConfig::new(
                    AuthOptions {
                        endpoint: Cow::Borrowed(&self.endpoint),
                        key: self.key.as_deref().map(Cow::Borrowed),
                        headers: Cow::Borrowed(self.headers.as_slice()),
                        token: self.auth_token.as_deref().map(Cow::Borrowed),
                        proxy: self.proxy.as_deref().map(Cow::Borrowed),
                    },
                    self.audio_format,
                )
                .connect(),
            )?,
        );
        Ok(())
    }

    fn speak_ssml(&self, ssml: &str) -> PyResult<()> {
        let buffer = self.runtime.block_on(
            self.synthesizer
                .borrow_mut()
                .as_mut()
                .ok_or(PyOSError::new_err("Synthesizer not connected"))?
                .synthesize_ssml(ssml),
        )?;
        play_owned_audio_blocking(buffer)?;
        Ok(())
    }

    #[pyo3(signature = (ssml, **options))]
    fn synthesize_ssml<'a>(
        &self,
        ssml: &str,
        options: Option<&PyDict>,
        py: Python<'a>,
    ) -> PyResult<Option<&'a PyBytes>> {
        let data = self.runtime.block_on(
            self.synthesizer
                .borrow_mut()
                .as_mut()
                .ok_or(PyOSError::new_err("Synthesizer not connected"))?
                .synthesize_ssml(ssml),
        )?;
        if let Some(output) = options
            .and_then(|d| d.get_item("output").map(|f| f.extract::<&str>()))
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
    fn speak_text(&self, text: &str, options: Option<&PyDict>) -> PyResult<()> {
        let buffer = self.runtime.block_on(
            self.synthesizer
                .borrow_mut()
                .as_mut()
                .ok_or(PyOSError::new_err("Synthesizer not connected"))?
                .synthesize_text(
                    text,
                    &Self::parse_text_options(options)?.unwrap_or_default(),
                ),
        )?;
        play_owned_audio_blocking(buffer)?;
        Ok(())
    }

    #[pyo3(signature = (text, **options))]
    fn synthesize_text<'a>(
        &self,
        text: &str,
        options: Option<&PyDict>,
        py: Python<'a>,
    ) -> PyResult<Option<&'a PyBytes>> {
        let data = self.runtime.block_on(
            self.synthesizer
                .borrow_mut()
                .as_mut()
                .ok_or(PyOSError::new_err("Synthesizer not connected"))?
                .synthesize_text(
                    text,
                    &Self::parse_text_options(options)?.unwrap_or_default(),
                ),
        )?;
        if let Some(output) = options
            .and_then(|d| d.get_item("output").map(|f| f.extract::<&str>()))
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
