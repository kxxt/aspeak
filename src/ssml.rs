use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::TextOptions;

use log::info;
use xml::{
    writer::{events::StartElementBuilder, XmlEvent},
    EventWriter,
};

trait StartElementBuilderExt<'a> {
    fn optional_attrs(self, attrs: &'a [(&str, Option<&str>)]) -> Self;
    fn optional_ns(self, cond: bool, ns: &'a str, uri: &'a str) -> Self;
}

impl<'a> StartElementBuilderExt<'a> for StartElementBuilder<'a> {
    fn optional_attrs(self, attrs: &'a [(&str, Option<&str>)]) -> Self {
        attrs.iter().fold(self, |acc, (name, value)| {
            if let Some(v) = value {
                acc.attr(*name, v)
            } else {
                acc
            }
        })
    }
    fn optional_ns(self, cond: bool, ns: &'a str, uri: &'a str) -> Self {
        if cond {
            self.ns(ns, uri)
        } else {
            self
        }
    }
}

const DEFAULT_PITCH_RATE_STR: &str = "0%";

/// Interpolate SSML from text and options
pub fn interpolate_ssml(text: impl AsRef<str>, options: &TextOptions) -> Result<String, SsmlError> {
    let mut buf = Vec::new();
    let mut writer = EventWriter::new_with_config(
        &mut buf,
        xml::EmitterConfig::new().write_document_declaration(false),
    );
    writer.write({
        XmlEvent::start_element("speak")
            .default_ns("http://www.w3.org/2001/10/synthesis")
            .optional_ns(
                options.rich_ssml_options.is_some(),
                "mstts",
                "http://www.w3.org/2001/mstts",
            )
            .ns("emo", "http://www.w3.org/2009/10/emotionml")
            .attr("version", "1.0")
            .attr("xml:lang", "en-US")
    })?;

    writer.write(XmlEvent::start_element("voice").attr("name", &options.voice))?;

    // Make the borrow checker happy
    if let Some(rich_ssml_options) = options.rich_ssml_options.as_ref() {
        let style_degree = rich_ssml_options.style_degree.map(|x| x.to_string());
        writer.write(
            XmlEvent::start_element("mstts:express-as")
                .optional_attrs(&[
                    ("role", rich_ssml_options.role.map(|role| role.into())),
                    ("styledegree", style_degree.as_deref()),
                ])
                .attr(
                    "style",
                    rich_ssml_options.style.as_deref().unwrap_or("general"),
                ),
        )?;
    }
    writer.write(
        XmlEvent::start_element("prosody")
            .attr(
                "pitch",
                options.pitch.as_deref().unwrap_or(DEFAULT_PITCH_RATE_STR),
            )
            .attr(
                "rate",
                options.rate.as_deref().unwrap_or(DEFAULT_PITCH_RATE_STR),
            ),
    )?;
    writer.write(XmlEvent::characters(text.as_ref()))?;
    writer.write(XmlEvent::end_element())?;
    if options.rich_ssml_options.is_some() {
        writer.write(XmlEvent::end_element())?;
    }
    writer.write(XmlEvent::end_element())?;
    writer.write(XmlEvent::end_element())?;
    let ssml = String::from_utf8(buf).unwrap();
    info!("Created SSML: {}", &ssml);
    Ok(ssml)
}

#[derive(Debug)]
#[non_exhaustive]
pub struct SsmlError {
    pub kind: SsmlErrorKind,
    pub(crate) source: Option<anyhow::Error>,
}

impl Display for SsmlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ssml {:?} error", self.kind)
    }
}

impl Error for SsmlError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as _)
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum SsmlErrorKind {
    Xml,
}

macro_rules! impl_from_for_ssml_error {
    ($error_type:ty, $error_kind:ident) => {
        impl From<$error_type> for SsmlError {
            fn from(e: $error_type) -> Self {
                Self {
                    kind: SsmlErrorKind::$error_kind,
                    source: Some(e.into()),
                }
            }
        }
    };
}

impl_from_for_ssml_error!(xml::writer::Error, Xml);
