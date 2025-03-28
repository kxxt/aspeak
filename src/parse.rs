use std::{
    borrow::Cow,
    error::Error,
    fmt::{self, Display, Formatter},
};

fn is_float(s: &str) -> bool {
    s.parse::<f32>().is_ok()
}

pub(crate) fn parse_pitch(arg: &str) -> Result<Cow<'static, str>, ParseError> {
    if (arg.ends_with("Hz") && is_float(&arg[..arg.len() - 2]))
        || (arg.ends_with('%') && is_float(&arg[..arg.len() - 1]))
        || (arg.ends_with("st")
            && (arg.starts_with('+') || arg.starts_with('-'))
            && is_float(&arg[..arg.len() - 2]))
        || ["default", "x-low", "low", "medium", "high", "x-high"].contains(&arg)
    {
        Ok(Cow::Owned(arg.to_string()))
    } else if let Ok(v) = arg.parse::<f32>() {
        // float values that will be converted to percentages
        Ok(format!("{:.2}%", v * 100f32).into())
    } else {
        Err(ParseError::new(format!(
            "Invalid pitch: {arg}. Please read the documentation for possible values of pitch."
        )))
    }
}

pub(crate) fn parse_rate(arg: &str) -> Result<Cow<'static, str>, ParseError> {
    if (arg.ends_with('%') && is_float(&arg[..arg.len() - 1]))
        || ["default", "x-slow", "slow", "medium", "fast", "x-fast"].contains(&arg)
    {
        Ok(Cow::Owned(arg.to_string()))
    } else if arg.ends_with('f') && is_float(&arg[..arg.len() - 1]) {
        // raw float
        Ok(Cow::Owned(arg[..arg.len() - 1].to_string()))
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_pitch_with_hz() {
        let input = "440Hz";
        let result = parse_pitch(input).unwrap();
        assert_eq!(result.as_ref(), "440Hz");
    }

    #[test]
    fn test_parse_pitch_with_percent_suffix() {
        let input = "50%";
        let result = parse_pitch(input).unwrap();
        // When the input ends with '%', it's already valid.
        assert_eq!(result.as_ref(), "50%");
    }

    #[test]
    fn test_parse_pitch_with_st() {
        let input = "+1.5st";
        let result = parse_pitch(input).unwrap();
        assert_eq!(result.as_ref(), "+1.5st");
    }

    #[test]
    fn test_parse_pitch_presets() {
        let presets = ["default", "x-low", "low", "medium", "high", "x-high"];
        for &preset in &presets {
            let result = parse_pitch(preset).unwrap();
            assert_eq!(result.as_ref(), preset);
        }
    }

    #[test]
    fn test_parse_pitch_float_conversion() {
        // Input is a float string without any suffix;
        // should be converted to a percentage.
        let input = "0.25";
        let result = parse_pitch(input).unwrap();
        assert_eq!(result.as_ref(), "25.00%");
    }

    #[test]
    fn test_parse_pitch_invalid_format() {
        let input = "invalid";
        let err = parse_pitch(input).unwrap_err();
        assert!(err.reason.contains("Invalid pitch:"));
    }

    #[test]
    fn test_parse_pitch_invalid_suffix() {
        let input = "100XYZ";
        let err = parse_pitch(input).unwrap_err();
        assert!(err.reason.contains("Invalid pitch:"));
    }

    // --- Tests for parse_rate ---

    #[test]
    fn test_parse_rate_with_percent() {
        let input = "75%";
        let result = parse_rate(input).unwrap();
        assert_eq!(result.as_ref(), "75%");
    }

    #[test]
    fn test_parse_rate_presets() {
        let presets = ["default", "x-slow", "slow", "medium", "fast", "x-fast"];
        for &preset in &presets {
            let result = parse_rate(preset).unwrap();
            assert_eq!(result.as_ref(), preset);
        }
    }

    #[test]
    fn test_parse_rate_raw_float_with_f() {
        let input = "0.5f";
        let result = parse_rate(input).unwrap();
        // The trailing 'f' is removed.
        assert_eq!(result.as_ref(), "0.5");
    }

    #[test]
    fn test_parse_rate_float_conversion() {
        // When input is a raw float string, it converts to percentage.
        let input = "0.8";
        let result = parse_rate(input).unwrap();
        assert_eq!(result.as_ref(), "80.00%");
    }

    #[test]
    fn test_parse_rate_invalid_format() {
        let input = "bad-rate";
        let err = parse_rate(input).unwrap_err();
        assert!(err.reason.contains("Invalid rate:"));
    }

    #[test]
    fn test_parse_rate_invalid_suffix() {
        let input = "0.5x";
        let err = parse_rate(input).unwrap_err();
        assert!(err.reason.contains("Invalid rate:"));
    }

    // --- Tests for parse_style_degree ---

    #[test]
    fn test_parse_style_degree_valid() {
        let input = "1.0";
        let result = parse_style_degree(input).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_parse_style_degree_lower_boundary() {
        let input = "0.01";
        let result = parse_style_degree(input).unwrap();
        assert_eq!(result, 0.01);
    }

    #[test]
    fn test_parse_style_degree_upper_boundary() {
        let input = "2.0";
        let result = parse_style_degree(input).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_parse_style_degree_out_of_range_low() {
        let input = "0.0";
        let err = parse_style_degree(input).unwrap_err();
        assert!(err.reason.contains("Invalid style degree value"));
    }

    #[test]
    fn test_parse_style_degree_out_of_range_high() {
        let input = "2.1";
        let err = parse_style_degree(input).unwrap_err();
        assert!(err.reason.contains("Invalid style degree value"));
    }

    #[test]
    fn test_parse_style_degree_not_a_float() {
        let input = "not-a-number";
        let err = parse_style_degree(input).unwrap_err();
        assert!(err.reason.contains("Invalid style degree:"));
    }

    // --- Tests for validate_style_degree ---

    #[test]
    fn test_validate_style_degree_within_range() {
        assert!(validate_style_degree(0.5));
    }

    #[test]
    fn test_validate_style_degree_at_boundaries() {
        assert!(validate_style_degree(0.01));
        assert!(validate_style_degree(2.0));
    }

    #[test]
    fn test_validate_style_degree_outside_range() {
        assert!(!validate_style_degree(0.0));
        assert!(!validate_style_degree(2.1));
    }
}
