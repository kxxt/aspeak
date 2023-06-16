use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use strum::AsRefStr;

#[derive(Debug)]
#[non_exhaustive]
/// Errors that can occur while connecting to the service
///
/// Possible reasons are
/// - A bad request is constructed somehow
/// - Proxy related errors
/// - Network related errors
/// - Bad response from the service
pub struct ConnectError {
    pub kind: ConnectErrorKind,
    pub(crate) source: Option<anyhow::Error>,
}

impl Display for ConnectError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "connect error: ")?;
        match self.kind {
            ConnectErrorKind::UnsupportedScheme(ref scheme) => {
                if let Some(ref scheme) = scheme {
                    write!(f, "unsupported proxy scheme: {}", scheme)
                } else {
                    write!(f, "no proxy scheme found in url")
                }
            }
            ConnectErrorKind::BadUrl(ref url) => write!(f, "bad url: {url}"),
            _ => write!(
                f,
                "{} error while connecting to proxy or azure API",
                self.kind.as_ref()
            ),
        }
    }
}

impl Error for ConnectError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as _)
    }
}

#[derive(Debug, PartialEq, Clone, AsRefStr)]
#[non_exhaustive]
#[allow(unused)]
#[strum(serialize_all = "title_case")]
pub enum ConnectErrorKind {
    BadUrl(String),
    UnsupportedScheme(Option<String>),
    RequestConstruction,
    BadResponse,
    Connection,
}

#[allow(unused_macros)]
macro_rules! impl_from_for_connect_error {
    ($error_type:ty, $error_kind:ident) => {
        impl From<$error_type> for ConnectError {
            fn from(e: $error_type) -> Self {
                Self {
                    kind: ConnectErrorKind::$error_kind,
                    source: Some(e.into()),
                }
            }
        }
    };
}

#[cfg(feature = "websocket-synthesizer")]
impl_from_for_connect_error!(tokio_tungstenite::tungstenite::Error, Connection);
#[cfg(feature = "websocket-synthesizer")]
impl_from_for_connect_error!(std::io::Error, Connection);
#[cfg(feature = "websocket-synthesizer")]
impl_from_for_connect_error!(tokio_socks::Error, Connection);
#[cfg(feature = "websocket-synthesizer")]
impl_from_for_connect_error!(hyper::Error, Connection);
