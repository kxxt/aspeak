use std::error::Error;

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

#[path = "../parse.rs"]
mod parse_common;

pub(crate) use parse_common::*;
