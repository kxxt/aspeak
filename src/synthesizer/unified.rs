use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use async_trait::async_trait;
use log::debug;
use strum::AsRefStr;

use crate::{interpolate_ssml, SsmlError, TextOptions};

#[async_trait]
pub trait UnifiedSynthesizer: Send {
    async fn process_ssml(&mut self, ssml: &str) -> Result<Vec<u8>, UnifiedSynthesizerError>;
    /// This is a convenience method that interpolates the SSML for you.
    async fn process_text(
        &mut self,
        text: &str,
        options: &TextOptions<'_>,
    ) -> Result<Vec<u8>, UnifiedSynthesizerError> {
        debug!("Synthesizing text: {}", text);
        let ssml = interpolate_ssml(text, options)?;
        self.process_ssml(&ssml).await
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct UnifiedSynthesizerError {
    pub kind: UnifiedSynthesizerErrorKind,
    pub(crate) source: Option<anyhow::Error>,
}

impl Display for UnifiedSynthesizerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use UnifiedSynthesizerErrorKind::*;
        write!(f, "unified synthesizer error: ")?;
        match &self.kind {
            Connect => write!(f, "error while connecting to the server"),
            InvalidRequest => write!(
                f,
                "an invalid request is constructed or 400 status reported by the server"
            ),
            _ => write!(f, "{} error", self.kind.as_ref()),
        }
    }
}

impl Error for UnifiedSynthesizerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as _)
    }
}

#[cfg(feature = "python")]
impl From<UnifiedSynthesizerError> for pyo3::PyErr {
    fn from(value: UnifiedSynthesizerError) -> Self {
        pyo3::exceptions::PyOSError::new_err(format!("{:?}", color_eyre::Report::from(value)))
    }
}

#[derive(Debug, PartialEq, Clone, AsRefStr)]
#[allow(unused)]
#[non_exhaustive]
#[strum(serialize_all = "title_case")]
pub enum UnifiedSynthesizerErrorKind {
    Connect,
    InvalidRequest,
    Http,
    Connection,
    InvalidMessage,
    Ssml,
}

macro_rules! impl_from_for_unified_synthesizer_error {
    ($error_type:ty, $error_kind:ident) => {
        impl From<$error_type> for UnifiedSynthesizerError {
            fn from(e: $error_type) -> Self {
                Self {
                    kind: UnifiedSynthesizerErrorKind::$error_kind,
                    source: Some(e.into()),
                }
            }
        }
    };
}

impl_from_for_unified_synthesizer_error!(SsmlError, Ssml);

#[cfg(feature = "rest-synthesizer")]
impl From<super::RestSynthesizerError> for UnifiedSynthesizerError {
    fn from(value: super::RestSynthesizerError) -> Self {
        use crate::synthesizer::RestSynthesizerErrorKind as RestKind;
        use UnifiedSynthesizerErrorKind::*;
        match &value.kind {
            RestKind::Connect => Self {
                kind: Connect,
                source: Some(value.into()),
            },
            RestKind::InvalidRequest => Self {
                kind: InvalidRequest,
                source: Some(value.into()),
            },
            RestKind::Unauthorized
            | RestKind::TooManyRequests
            | RestKind::UnsupportedMediaType
            | RestKind::OtherHttp => Self {
                kind: Http,
                source: Some(value.into()),
            },
            RestKind::Connection => Self {
                kind: Connection,
                source: Some(value.into()),
            },
            RestKind::Ssml => Self {
                kind: Ssml,
                source: Some(value.into()),
            },
        }
    }
}

#[cfg(feature = "websocket-synthesizer")]
impl From<super::WebsocketSynthesizerError> for UnifiedSynthesizerError {
    fn from(value: super::WebsocketSynthesizerError) -> Self {
        use crate::synthesizer::WebsocketSynthesizerErrorKind as WsKind;
        use UnifiedSynthesizerErrorKind::*;
        match &value.kind {
            WsKind::Connect => Self {
                kind: Connect,
                source: Some(value.into()),
            },
            WsKind::WebsocketConnectionClosed { code: _, reason: _ } => Self {
                kind: Connection,
                source: Some(value.into()),
            },
            WsKind::Websocket => Self {
                kind: Connection,
                source: Some(value.into()),
            },
            WsKind::InvalidRequest => Self {
                kind: InvalidRequest,
                source: Some(value.into()),
            },
            WsKind::InvalidMessage => Self {
                kind: InvalidMessage,
                source: Some(value.into()),
            },
            WsKind::Ssml => Self {
                kind: Ssml,
                source: Some(value.into()),
            },
        }
    }
}

#[cfg(feature = "rest-synthesizer")]
#[async_trait]
impl UnifiedSynthesizer for super::RestSynthesizer {
    async fn process_ssml(&mut self, ssml: &str) -> Result<Vec<u8>, UnifiedSynthesizerError> {
        Ok(self.synthesize_ssml(ssml).await?)
    }
}

#[cfg(feature = "websocket-synthesizer")]
#[async_trait]
impl UnifiedSynthesizer for super::WebsocketSynthesizer {
    async fn process_ssml(&mut self, ssml: &str) -> Result<Vec<u8>, UnifiedSynthesizerError> {
        Ok(self.synthesize_ssml(ssml).await?)
    }
}
