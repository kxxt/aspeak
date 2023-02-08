use std::{borrow::Cow, error::Error};

use reqwest::header::{HeaderName, HeaderValue};

/// Parse a single key-value pair
pub(super) fn parse_header(
    s: &str,
) -> Result<(HeaderName, HeaderValue), Box<dyn Error + Send + Sync + 'static>> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((
        HeaderName::from_bytes(s[..pos].as_bytes())?,
        HeaderValue::from_str(&s[pos + 1..])?,
    ))
}

fn is_float(s: &str) -> bool {
    return s.parse::<f32>().is_ok();
}

pub(super) fn parse_pitch<'a>(arg: &'a str) -> Result<Cow<'a, str>, String> {
    if (arg.ends_with("Hz") && is_float(&arg[..arg.len() - 2]))
        || (arg.ends_with("%") && is_float(&arg[..arg.len() - 1]))
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
        Err(format!(
            "Please read the documentation for possible values of pitch."
        ))
    }
}

pub(super) fn parse_rate<'a>(arg: &'a str) -> Result<Cow<'a, str>, String> {
    if (arg.ends_with("%") && is_float(&arg[..arg.len() - 1]))
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
        Err(format!(
            "Please read the documentation for possible values of rate."
        ))
    }
}

pub(super) fn parse_style_degree(arg: &str) -> Result<f32, String> {
    if let Ok(v) = arg.parse::<f32>() {
        if validate_style_degree(v) {
            Ok(v)
        } else {
            Err(format!("Value {v} out of range [0.01, 2]"))
        }
    } else {
        Err("Not a floating point number!".to_owned())
    }
}

pub(super) fn validate_style_degree(degree: f32) -> bool {
    0.01f32 <= degree && degree <= 2.0f32
}
