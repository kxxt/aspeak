use crate::error::Result;
use crate::TextOptions;

use log::info;
use xml::{
    writer::{events::StartElementBuilder, XmlEvent},
    EventWriter,
};

trait StartElementBuilderExt<'a> {
    fn optional_attrs(self, attrs: &'a [(&str, Option<&str>)]) -> Self;
}

impl<'a> StartElementBuilderExt<'a> for StartElementBuilder<'a> {
    fn optional_attrs(self, attrs: &'a [(&str, Option<&str>)]) -> Self {
        attrs.into_iter().fold(self, |acc, (name, value)| {
            if let Some(ref v) = value {
                acc.attr(*name, v)
            } else {
                acc
            }
        })
    }
}

const DEFAULT_PITCH_RATE_STR: &str = "0%";

pub fn interpolate_ssml(text: impl AsRef<str>, options: &TextOptions) -> Result<String> {
    let mut buf = Vec::new();
    let mut writer = EventWriter::new_with_config(
        &mut buf,
        xml::EmitterConfig::new().write_document_declaration(false),
    );
    writer.write(
        XmlEvent::start_element("speak")
            .default_ns("http://www.w3.org/2001/10/synthesis")
            .ns("mstts", "http://www.w3.org/2001/mstts")
            .ns("emo", "http://www.w3.org/2009/10/emotionml")
            .attr("version", "1.0")
            .attr("xml:lang", "en-US"),
    )?;

    writer.write(XmlEvent::start_element("voice").attr("name", &options.voice))?;

    // Make the borrow checker happy
    let style_degree = options.style_degree.map(|x| x.to_string());
    writer.write(
        XmlEvent::start_element("mstts:express-as")
            .optional_attrs(&[
                ("role", options.role.map(|role| role.into())),
                ("styledegree", style_degree.as_deref()),
            ])
            .attr("style", options.style.as_deref().unwrap_or("general")),
    )?;
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
    writer.write(XmlEvent::end_element())?;
    writer.write(XmlEvent::end_element())?;
    writer.write(XmlEvent::end_element())?;
    let ssml = String::from_utf8(buf).unwrap();
    info!("Created SSML: {}", &ssml);
    return Ok(ssml);
}
