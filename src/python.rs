use pyo3::exceptions::PyValueError;
use pyo3::{prelude::*, types::PyDict};

use crate::{
    callback_play_blocking, AudioFormat, Synthesizer, SynthesizerConfig, DEFAULT_ENDPOINT,
};

#[pymodule]
fn aspeak(py: Python, m: &PyModule) -> PyResult<()> {
    crate::types::register_python_items(py, m)?;
    crate::synthesizer::register_python_items(py, m)?;
    Ok(())
}

#[pyclass(name = "SynthesizerConfig")]
struct PySynthesizerConfig();

impl PySynthesizerConfig {}

#[pymethods]
impl PySynthesizerConfig {
    #[new]
    #[pyo3(signature = (audio_format,**options))]
    fn new(audio_format: AudioFormat, options: Option<&PyDict>) -> PyResult<Self> {
        let endpoint = options
            .and_then(|dict| dict.get_item("endpoint"))
            .map(|endpoint| endpoint.extract::<String>())
            .transpose()?;
        Ok(Self {})
    }

    fn connect(&self) -> PyResult<Synthesizer> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        let config = self.clone();
        todo!()
        // Ok(rt.block_on(config.connect())?)
    }
}

#[pymethods]
impl Synthesizer {
    fn speak_ssml(&self, ssml: &str) -> PyResult<()> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        Ok(rt.block_on(self.synthesize(ssml, callback_play_blocking()))?)
    }

    fn speak_text(&self, text: &str, options: Option<&PyDict>) -> PyResult<()> {
        todo!()
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
