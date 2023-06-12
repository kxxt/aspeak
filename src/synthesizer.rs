use log::info;

use crate::{AudioFormat, AuthOptions};

#[cfg(feature = "rest-synthesizer")]
mod rest;
#[cfg(feature = "unified-synthesizer")]
mod unified;
#[cfg(feature = "websocket-synthesizer")]
mod websocket;

#[cfg(feature = "rest-synthesizer")]
pub use rest::*;
#[cfg(feature = "websocket-synthesizer")]
pub use websocket::*;

/// Initialize a new [`Synthesizer`] by creating a new [`SynthesizerConfig`] and call [`SynthesizerConfig::connect`].
#[derive(Debug, Clone)]
#[allow(unused)]
pub struct SynthesizerConfig<'a> {
    /// The authentication options.
    pub(crate) auth: AuthOptions<'a>,
    /// The audio format of the output audio.
    pub(crate) audio_format: AudioFormat,
}

#[cfg(feature = "websocket-synthesizer")]
const CLIENT_INFO_PAYLOAD: &str = r#"{"context":{"system":{"version":"1.25.0","name":"SpeechSDK","build":"Windows-x64"},"os":{"platform":"Windows","name":"Client","version":"10"}}}"#; // r#"{"context":{"system":{"name":"SpeechSDK","version":"1.12.1-rc.1","build":"JavaScript","lang":"JavaScript","os":{"platform":"Browser/Linux x86_64","name":"Mozilla/5.0 (X11; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0","version":"5.0 (X11)"}}}}"#;

impl<'a> SynthesizerConfig<'a> {
    /// Create a new [`SynthesizerConfig`] with the given [`AuthOptions`] and [`AudioFormat`].
    pub fn new(auth: AuthOptions<'a>, audio_format: AudioFormat) -> Self {
        info!("Successfully created SynthesizerConfig");
        Self { auth, audio_format }
    }

    #[cfg(feature = "websocket-synthesizer")]
    fn generate_client_request(
        &self,
    ) -> Result<tokio_tungstenite::tungstenite::handshake::client::Request, WebsocketSynthesizerError>
    {
        use hyper::http::HeaderValue;
        use log::debug;
        use tokio_tungstenite::tungstenite::client::IntoClientRequest;
        use uuid::Uuid;

        use crate::{constants::ORIGIN, DEFAULT_ENDPOINT};

        let uuid = Uuid::new_v4();
        let request_id = uuid.as_simple().to_string();
        let uri = {
            let mut url = url::Url::parse(&self.auth.endpoint)?;
            url.query_pairs_mut()
                .append_pair("X-ConnectionId", &request_id);
            if let Some(auth_token) = &self.auth.token {
                url.query_pairs_mut()
                    .append_pair("Authorization", auth_token);
            }
            url
        };
        let mut request = uri
            .into_client_request()
            .map_err(|e| WebsocketSynthesizerError {
                kind: WebsocketSynthesizerErrorKind::InvalidRequest,
                source: Some(e.into()),
            })?;
        let headers = request.headers_mut();
        if let Some(key) = &self.auth.key {
            headers.append("Ocp-Apim-Subscription-Key", HeaderValue::from_str(key)?);
        }
        if !self.auth.headers.is_empty() {
            // TODO: I don't know if this could be further optimized
            headers.extend(self.auth.headers.iter().map(Clone::clone));
        } else if Some(self.auth.endpoint.as_ref()) == DEFAULT_ENDPOINT {
            // Trial endpoint
            headers.append("Origin", HeaderValue::from_str(ORIGIN).unwrap());
        }
        debug!("The initial request is {request:?}");
        Ok(request)
    }

    /// Connect to the Azure Speech Service and return a [`Synthesizer`] on success.
    #[cfg(feature = "websocket-synthesizer")]
    pub async fn connect_websocket(
        self,
    ) -> Result<WebsocketSynthesizer, WebsocketSynthesizerError> {
        use crate::errors::{ConnectError, ConnectErrorKind};
        use crate::net::{self, connect_directly};
        use chrono::Utc;
        use futures_util::SinkExt;
        use tokio_tungstenite::tungstenite::Message;
        use uuid::Uuid;

        let request = self.generate_client_request()?;
        let proxy_url = self
            .auth
            .proxy
            .as_deref()
            .map(reqwest::Url::parse)
            .transpose()
            .map_err(|e| ConnectError {
                kind: ConnectErrorKind::BadUrl(self.auth.proxy.unwrap().to_string()),
                source: Some(e.into()),
            })?;
        let mut wss = match proxy_url.as_ref().map(|x| x.scheme()) {
            Some("socks5") => {
                net::connect_via_socks5_proxy(request, proxy_url.as_ref().unwrap()).await?
            }
            Some("http") | Some("https") => {
                net::connect_via_http_proxy(request, proxy_url.as_ref().unwrap()).await?
            }
            None => connect_directly(request).await?,
            Some(other_scheme) => {
                return Err(ConnectError {
                    kind: ConnectErrorKind::UnsupportedScheme(Some(other_scheme.to_string())),
                    source: None,
                }
                .into())
            }
        };
        let uuid = Uuid::new_v4();
        let request_id = uuid.as_simple();
        let now = Utc::now();
        wss.send(Message::Text(format!(
            "Path: speech.config\r\nX-RequestId: {request_id}\r\nX-Timestamp: {now:?}Content-Type: application/json\r\n\r\n{CLIENT_INFO_PAYLOAD}"
        ))).await?;
        info!("Successfully created Synthesizer");
        Ok(WebsocketSynthesizer {
            audio_format: self.audio_format,
            stream: wss,
        })
    }

    #[cfg(feature = "rest-synthesizer")]
    pub fn rest_synthesizer(&self) -> Result<RestSynthesizer, RestSynthesizerError> {
        use crate::utils::{transpose_tuple_option_result, ClientBuilderExt};
        use hyper::{header, http::HeaderValue};
        use reqwest::Proxy;

        Ok(RestSynthesizer {
            client: reqwest::Client::builder()
                .user_agent("aspeak")
                .default_headers(header::HeaderMap::from_iter(
                    [
                        Some((
                            header::CONTENT_TYPE,
                            HeaderValue::from_static("application/ssml+xml"),
                        )),
                        Some((
                            header::HeaderName::from_static("X-Microsoft-OutputFormat"),
                            HeaderValue::from_static(self.audio_format.into()),
                        )),
                        transpose_tuple_option_result(self.auth.key().map(|key| {
                            (
                                header::HeaderName::from_static("Ocp-Apim-Subscription-Key"),
                                HeaderValue::from_str(key),
                            )
                        }))?,
                        transpose_tuple_option_result(self.auth.token().map(|token| {
                            (
                                header::HeaderName::from_static("Authorization"),
                                HeaderValue::from_str(token),
                            )
                        }))?,
                    ]
                    .into_iter()
                    .flatten()
                    .chain(self.auth.headers.iter().map(Clone::clone)),
                ))
                .optional_proxy(
                    self.auth
                        .proxy
                        .as_deref()
                        .map(Proxy::all)
                        .transpose()
                        .map_err(|e| RestSynthesizerError {
                            kind: RestSynthesizerErrorKind::Connect,
                            source: Some(e.into()),
                        })?,
                )
                .build()
                .map_err(|e| RestSynthesizerError {
                    kind: RestSynthesizerErrorKind::Connect,
                    source: Some(e.into()),
                })?,
            endpoint: self.auth.endpoint.to_string(),
        })
    }
}
