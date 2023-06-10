use std::{
    borrow::Cow,
    error::Error,
    fmt::{self, Display, Formatter},
};

fn is_float(s: &str) -> bool {
    s.parse::<f32>().is_ok()
}

pub(crate) fn parse_pitch(arg: &str) -> Result<Cow<'_, str>, ParseError> {
    if (arg.ends_with("Hz") && is_float(&arg[..arg.len() - 2]))
        || (arg.ends_with('%') && is_float(&arg[..arg.len() - 1]))
        || (arg.ends_with("st")
            && (arg.starts_with('+') || arg.starts_with('-'))
            && is_float(&arg[..arg.len() - 2]))
        || ["default", "x-low", "low", "medium", "high", "x-high"].contains(&arg)
    {
        Ok(Cow::Borrowed(arg))
    } else if let Ok(v) = arg.parse::<f32>() {
        // float values that will be converted to percentages
        Ok(Cow::Owned(format!("{:.2}%", v * 100f32)))
    } else {
        Err(ParseError::new(format!(
            "Invalid pitch: {arg}. Please read the documentation for possible values of pitch."
        )))
    }
}

pub(crate) fn parse_rate(arg: &str) -> Result<Cow<'_, str>, ParseError> {
    if (arg.ends_with('%') && is_float(&arg[..arg.len() - 1]))
        || ["default", "x-slow", "slow", "medium", "fast", "x-fast"].contains(&arg)
    {
        Ok(Cow::Borrowed(arg))
    } else if arg.ends_with('f') && is_float(&arg[..arg.len() - 1]) {
        // raw float
        Ok(Cow::Borrowed(&arg[..arg.len() - 1]))
    } else if let Ok(v) = arg.parse::<f32>() {
        // float values that will be converted to percentages
        Ok(Cow::Owned(format!("{:.2}%", v * 100f32)))
    } else {
        Err(ParseError::new(format!(
            "Invalid rate: {arg}. Please read the documentation for possible values of rate."
        )))
    }
}

pub(crate) fn parse_style_degree(arg: &str) -> Result<f32, ParseError> {
    if let Ok(v) = arg.parse::<f32>() {
        if validate_style_degree(v) {
            Ok(v)
        } else {
            Err(ParseError::new(format!(
                "Invalid style degree value {v}! out of range [0.01, 2]"
            )))
        }
    } else {
        Err(ParseError::new(format!(
            "Invalid style degree: {arg}Not a floating point number!"
        )))
    }
}

pub(crate) fn validate_style_degree(degree: f32) -> bool {
    (0.01f32..=2.0f32).contains(&degree)
}

#[derive(Debug)]
#[non_exhaustive]
pub struct ParseError {
    pub reason: String,
    pub(crate) source: Option<anyhow::Error>,
}

impl ParseError {
    pub(crate) fn new(reason: String) -> Self {
        Self {
            reason,
            source: None,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "parse error: {}", self.reason)
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as _)
    }
}

#[cfg(feature = "python")]
mod python {
    use color_eyre::eyre::Report;
    use pyo3::exceptions::PyValueError;
    use pyo3::prelude::*;

    impl From<super::ParseError> for PyErr {
        fn from(value: super::ParseError) -> Self {
            PyValueError::new_err(format!("{:?}", Report::from(value)))
        }
    }
}
