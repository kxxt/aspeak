use std::borrow::Cow;
use std::cell::RefCell;

use pyo3::exceptions::{PyOSError, PyValueError};
use pyo3::types::{PyIterator, PyList, PySequence};
use pyo3::{prelude::*, types::PyDict};
use reqwest::header::{HeaderName, HeaderValue};
use tokio::runtime::Runtime;

use crate::parse::{parse_pitch, parse_rate, parse_style_degree};
use crate::{
    callback_play_blocking, get_default_voice_by_locale, get_endpoint_by_region, interpolate_ssml,
    AudioFormat, AuthOptions, Synthesizer, SynthesizerConfig, TextOptions, DEFAULT_ENDPOINT,
};

#[pymodule]
fn aspeak(py: Python, m: &PyModule) -> PyResult<()> {
    crate::types::register_python_items(py, m)?;
    crate::synthesizer::register_python_items(py, m)?;
    Ok(())
}

#[pyclass]
struct SpeechService {
    audio_format: AudioFormat,
    endpoint: String,
    key: Option<String>,
    auth_token: Option<String>,
    headers: Vec<(HeaderName, HeaderValue)>,
    synthesizer: RefCell<Option<Synthesizer>>,
    runtime: Runtime,
}

impl SpeechService {}

#[pymethods]
impl SpeechService {
    #[new]
    #[pyo3(signature = (audio_format, **options))]
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
                .unwrap_or(Cow::Borrowed(DEFAULT_ENDPOINT))
        };
        let key: Option<String> = options
            .and_then(|dict| dict.get_item("key"))
            .map(|k| k.extract())
            .transpose()?;
        let token: Option<String> = options
            .and_then(|dict| dict.get_item("token"))
            .map(|k| k.extract())
            .transpose()?;
        let headers = options
            .and_then(|dict| dict.get_item("headers"))
            .map(|h| h.downcast::<PyIterator>())
            .transpose()?;
        let headers = if let Some(headers) = headers {
            headers
                .map(|header| {
                    header.and_then(|header| {
                        let header = header.downcast::<PySequence>()?;
                        let name = header.get_item(0)?.extract::<&str>()?;
                        let value = header.get_item(1)?.extract::<&str>()?;
                        Ok((
                            HeaderName::from_bytes(name.as_bytes()).map_err(|e| {
                                PyValueError::new_err(format!("Invalid header name: {}", e))
                            })?,
                            HeaderValue::from_str(value).map_err(|e| {
                                PyValueError::new_err(format!("Invalid header value: {}", e))
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
                        key: self.key.as_deref().map(|s| Cow::Borrowed(s)),
                        headers: Cow::Borrowed(self.headers.as_slice()),
                        token: self.auth_token.as_deref().map(Cow::Borrowed),
                    },
                    self.audio_format,
                )
                .connect(),
            )?,
        );
        Ok(())
    }

    #[pyo3(signature = (text, **options))]
    fn speak_text(&self, text: &str, options: Option<&PyDict>) -> PyResult<()> {
        self.runtime.block_on(
            self.synthesizer
                .borrow()
                .as_ref()
                .ok_or(PyOSError::new_err("Synthesizer not connected"))?
                .synthesize_text(
                    text,
                    &options
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
                                        Cow::Borrowed(get_default_voice_by_locale(locale)?)
                                    }
                                },
                                style: opts
                                    .get_item("style")
                                    .map(|s| s.extract())
                                    .transpose()?
                                    .map(Cow::Borrowed),
                                style_degree: opts
                                    .get_item("style_degree")
                                    .map(|l| l.extract())
                                    .transpose()?
                                    .map(parse_style_degree)
                                    .transpose()?,
                                role: opts.get_item("role").map(|r| r.extract()).transpose()?,
                            })
                        })
                        .transpose()?
                        .unwrap(),
                    callback_play_blocking(),
                ),
        )?;
        Ok(())
    }
}

// #[pymethods]
// impl TextArgs {
//     #[new]
//     #[pyo3(signature = (**kwargs))]
//     fn new(kwargs: Option<&PyDict>) -> PyResult<Self> {
//         let mut options = Self::default();
//         if let Some(dict) = kwargs {
//             if let Some(text) = dict.get_item("text") {
//                 options.text = Some(text.extract()?);
//             }
//             if let Some(pitch) = dict.get_item("pitch") {
//                 options.pitch =
//                     Some(parse_pitch(pitch.extract()?).map_err(|e| PyValueError::new_err(e))?);
//             }
//             if let Some(rate) = dict.get_item("rate") {
//                 options.rate =
//                     Some(parse_rate(rate.extract()?).map_err(|e| PyValueError::new_err(e))?);
//             }
//             if let Some(style) = dict.get_item("style") {
//                 options.style = Some(style.extract()?);
//             }
//             if let Some(role) = dict.get_item("role") {
//                 options.role = Some(role.extract()?);
//             }
//             if let Some(style_degree) = dict.get_item("style_degree") {
//                 let degree: f32 = style_degree.extract()?;
//                 if !validate_style_degree(degree) {
//                     return Err(PyValueError::new_err("Style degree out of range [0.01, 2]"));
//                 }
//                 options.style_degree = Some(degree);
//             }
//             if let Some(locale) = dict.get_item("locale") {
//                 // todo: default voice for locale
//                 options.locale = Some(locale.extract()?);
//             }
//             if let Some(voice) = dict.get_item("voice") {
//                 options.voice = Some(voice.extract()?);
//             }
//         }
//         Ok(options)
//     }
// }
