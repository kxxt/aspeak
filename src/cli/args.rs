use std::borrow::Cow;

use super::config::{AuthConfig, Config, OutputConfig};
use super::parse;
use aspeak::{
    get_endpoint_by_region, AudioFormat, AuthOptions, Role, DEFAULT_ENDPOINT, QUALITY_MAP,
};
use clap::{ArgAction, Args, ValueEnum};
use color_eyre::eyre::anyhow;
use reqwest::header::{HeaderName, HeaderValue};
use serde::Deserialize;
use strum::AsRefStr;

#[derive(Debug, Clone, Copy, Default, ValueEnum, AsRefStr, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub(crate) enum ContainerFormat {
    Mp3,
    Ogg,
    Webm,
    #[default]
    Wav,
}

#[derive(Args, Debug)]
pub struct ProfileArgs {
    #[arg(long, action = ArgAction::SetTrue, help = "Do not use profile")]
    no_profile: bool,
    #[arg(long, conflicts_with = "no_profile", help = "The profile to use")]
    profile: Option<String>,
}

impl ProfileArgs {
    pub(crate) fn load_profile(&self) -> color_eyre::Result<Option<Config>> {
        if self.no_profile {
            Ok(None)
        } else {
            Ok(Config::load(self.profile.as_ref())?)
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct AuthArgs {
    #[arg(short, long, help = "Endpoint of TTS API")]
    pub endpoint: Option<String>,
    #[arg(
        short,
        long,
        help = "If you are using official endpoints, you can specify a region instead of full endpoint url",
        conflicts_with = "endpoint"
    )]
    pub region: Option<String>,
    #[arg(short, long, help = "Auth token for speech service")]
    pub token: Option<String>,
    #[arg(short, long, help = "Speech resource key")]
    pub key: Option<String>,
    #[arg(short = 'H', long, value_parser = parse::parse_header, help = "Additional request headers")]
    pub headers: Vec<(HeaderName, HeaderValue)>,
}

impl AuthArgs {
    pub(crate) fn to_auth_options<'a>(
        &'a self,
        auth_config: Option<&'a AuthConfig>,
    ) -> color_eyre::Result<AuthOptions<'a>> {
        Ok(AuthOptions {
            endpoint: self
                .endpoint
                .as_deref()
                .map(Cow::Borrowed)
                .or_else(|| {
                    self.region
                        .as_deref()
                        .map(get_endpoint_by_region)
                        .map(Cow::Owned)
                })
                .or_else(|| {
                    auth_config
                        .map(|c| c.endpoint.as_ref().map(Cow::from))
                        .flatten()
                })
                .unwrap_or(Cow::Borrowed(DEFAULT_ENDPOINT)),
            token: match (self.token.as_deref(), auth_config) {
                (Some(token), _) => Some(Cow::Borrowed(token)),
                (None, Some(config)) => config.token.as_ref().map(Cow::from),
                (None, None) => None,
            },
            key: match (self.key.as_deref(), auth_config) {
                (Some(key), _) => Some(Cow::Borrowed(key)),
                (None, Some(config)) => config.key.as_ref().map(Cow::from),
                (None, None) => None,
            },
            headers: if let Some(AuthConfig {
                headers: Some(headers),
                ..
            }) = auth_config
            {
                let vec: color_eyre::Result<Vec<(HeaderName, HeaderValue)>> = headers
                    .iter()
                    .map(|(k, v)| {
                        Ok((
                            HeaderName::from_bytes(k.as_bytes())?,
                            HeaderValue::from_bytes(v.as_bytes())?,
                        ))
                    })
                    .collect();
                let mut vec = vec?;
                vec.extend_from_slice(&self.headers);
                Cow::Owned(vec)
            } else {
                Cow::Borrowed(&self.headers)
            },
        })
    }
}

#[derive(Args, Debug, Default)]
pub(crate) struct InputArgs {
    #[arg(short, long, help = "Text/SSML file to speak, default to `-`(stdin)")]
    pub file: Option<String>,
    #[arg(short, long, help = "Text/SSML file encoding")]
    pub encoding: Option<String>,
}

#[derive(Args, Debug, Default)]
pub(crate) struct OutputArgs {
    #[arg(short, long, help = "Output file path")]
    pub output: Option<String>,
    #[arg(
        short,
        long,
        allow_negative_numbers = true,
        help = "Output quality, default to 0. Run `aspeak list-qualities` to list available quality levels"
    )]
    pub quality: Option<i32>,
    #[arg(short, long)]
    pub container_format: Option<ContainerFormat>,
    #[arg(
        short = 'F',
        long,
        conflicts_with = "quality",
        conflicts_with = "container_format",
        hide_possible_values = true,
        help = "Set output audio format (experts only). Run `aspeak list-formats` to list available formats"
    )]
    pub format: Option<AudioFormat>,
}

impl OutputArgs {
    pub(crate) fn get_audio_format(
        &self,
        config: Option<&OutputConfig>,
    ) -> color_eyre::Result<AudioFormat> {
        Ok(
            match (
                self.format,
                self.container_format,
                self.quality,
                config
                    .map(|c| (c.format.as_ref(), c.container.as_ref(), c.quality.as_ref()))
                    .unwrap_or((None, None, None)),
            ) {
                // Explicitly specified format
                (Some(format), _, _, _) => format,
                // Explicitly specified container
                (None, Some(container), quality, (_, _, alt_quality)) => QUALITY_MAP
                    .get(container.as_ref())
                    .unwrap()
                    .get(&(quality.or(alt_quality.map(|x| *x)).unwrap_or_default() as i8))
                    .map(|x| *x)
                    .ok_or_else(|| {
                        anyhow!(format!(
                            "Invalid quality {:?} for container type {}",
                            quality,
                            container.as_ref()
                        ))
                    })?,
                // Explicitly specified quality
                (None, None, Some(quality), (_, alt_container, _)) => QUALITY_MAP
                    .get(alt_container.map(|x| *x).unwrap_or_default().as_ref())
                    .unwrap()
                    .get(&(quality as i8))
                    .map(|x| *x)
                    .ok_or_else(|| {
                        anyhow!(format!(
                            "Invalid quality {:?} for container type {}",
                            quality,
                            alt_container.unwrap_or(&ContainerFormat::Wav).as_ref()
                        ))
                    })?,
                // Format from config
                (None, None, None, (Some(format), _, _)) => *format,
                // Container and/or quality from config
                (None, None, None, (None, container, quality)) => QUALITY_MAP
                    .get(container.map(|x| *x).unwrap_or_default().as_ref())
                    .unwrap()
                    .get(&(quality.map(|x| *x).unwrap_or_default() as i8))
                    .map(|x| *x)
                    .ok_or_else(|| {
                        anyhow!(format!(
                            "Invalid quality {:?} for container type {:?}",
                            quality, container
                        ))
                    })?,
            },
        )
    }
}

fn parse_pitch(pitch: &str) -> Result<String, String> {
    parse::parse_pitch(pitch).map(String::from)
}

fn parse_rate(rate: &str) -> Result<String, String> {
    parse::parse_rate(rate).map(String::from)
}

#[derive(Args, Debug, Default)]
pub(crate) struct TextArgs {
    #[clap(help = "The text to speak. \
                If neither text nor input file is specified, the text will be read from stdin.")]
    pub text: Option<String>,
    #[arg(short, long, value_parser = parse_pitch,
        help="Set pitch, default to 0. \
              Valid values include floats(will be converted to percentages), \
              percentages such as 20% and -10%, absolute values like 300Hz, \
              and relative values like -20Hz, +2st and string values like x-low. \
              See the documentation for more details.")]
    pub pitch: Option<String>,
    #[arg(short, long, value_parser = parse_rate ,
        help=r#"Set speech rate, default to 0. \
                Valid values include floats(will be converted to percentages), \
                percentages like -20%%, floats with postfix "f" \
                (e.g. 2f means doubling the default speech rate), \
                and string values like x-slow. See the documentation for more details."# )]
    pub rate: Option<String>,
    #[arg(short = 'S', long, help = r#"Set speech style, default to "general""#)]
    pub style: Option<String>,
    #[arg(short = 'R', long)]
    pub role: Option<Role>,
    #[arg(
        short = 'd',
        long,
        value_parser = parse::parse_style_degree,
        help = "Specifies the intensity of the speaking style. This only works for some Chinese voices!"
    )]
    pub style_degree: Option<f32>,
    #[arg(short, long, conflicts_with = "locale", help = "Voice to use")]
    pub voice: Option<String>,
    #[arg(short, long, help = "Locale to use, default to en-US")]
    pub locale: Option<String>,
}
