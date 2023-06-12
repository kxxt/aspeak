use std::{
    pin::Pin,
    task::{Context, Poll},
};

use hyper::Uri;
use log::debug;
use reqwest::Url;
use tokio::{
    io::{AsyncRead, AsyncWrite, ReadBuf},
    net::TcpStream,
};

use tokio_socks::tcp::Socks5Stream;
use tokio_tungstenite::{tungstenite::client::IntoClientRequest, MaybeTlsStream, WebSocketStream};

use crate::errors::{ConnectError, ConnectErrorKind};

pub(crate) type WsStream = WebSocketStream<MaybeTlsStream<MaybeSocks5Stream<TcpStream>>>;

#[derive(Debug)]
pub(crate) enum MaybeSocks5Stream<S: AsyncRead + AsyncWrite + Unpin> {
    Plain(S),
    Socks5Stream(Socks5Stream<S>),
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncRead for MaybeSocks5Stream<S> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            MaybeSocks5Stream::Plain(ref mut s) => Pin::new(s).poll_read(cx, buf),
            MaybeSocks5Stream::Socks5Stream(s) => Pin::new(s).poll_read(cx, buf),
        }
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> AsyncWrite for MaybeSocks5Stream<S> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::result::Result<usize, std::io::Error>> {
        match self.get_mut() {
            MaybeSocks5Stream::Plain(ref mut s) => Pin::new(s).poll_write(cx, buf),
            MaybeSocks5Stream::Socks5Stream(s) => Pin::new(s).poll_write(cx, buf),
        }
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::result::Result<(), std::io::Error>> {
        match self.get_mut() {
            MaybeSocks5Stream::Plain(ref mut s) => Pin::new(s).poll_flush(cx),
            MaybeSocks5Stream::Socks5Stream(s) => Pin::new(s).poll_flush(cx),
        }
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::result::Result<(), std::io::Error>> {
        match self.get_mut() {
            MaybeSocks5Stream::Plain(ref mut s) => Pin::new(s).poll_shutdown(cx),
            MaybeSocks5Stream::Socks5Stream(s) => Pin::new(s).poll_shutdown(cx),
        }
    }
}

trait UriExt {
    fn host_and_port(&self) -> Result<(&str, u16), ConnectError>;
    fn host_colon_port(&self) -> Result<String, ConnectError> {
        let (host, port) = self.host_and_port()?;
        Ok(format!("{}:{}", host, port))
    }
}

macro_rules! impl_uri_ext {
    (@, Uri, $x:expr) => {
        $x
    };
    (@, Url, $x:expr) => {
        Some($x)
    };
    (@@, Uri, $x:ident) => {
        $x.host()
    };
    (@@, Url,$x:ident) => {
        $x.host_str()
    };
    ($type:ident, $port_method:ident, $scheme_method:ident) => {
        impl UriExt for $type {
            fn host_and_port(&self) -> Result<(&str, u16), ConnectError> {
                let port = match (self.$port_method(), impl_uri_ext!(@, $type, self.$scheme_method())) {
                    (Some(port), _) => port,
                    (None, Some("wss") | Some("https")) => 443,
                    (None, Some("ws") | Some("http")) => 80,
                    x => {
                        return Err(ConnectError {
                            kind: ConnectErrorKind::UnsupportedScheme(
                                x.1.map(|s| s.to_string()),
                            ),
                            source: None,
                        })
                    }
                };
                let host = impl_uri_ext!(@@, $type, self).ok_or_else(|| ConnectError {
                    kind: ConnectErrorKind::BadUrl(self.to_string()),
                    source: None,
                })?;
                Ok((host, port))
            }
        }
    };
}

impl_uri_ext!(Uri, port_u16, scheme_str);
impl_uri_ext!(Url, port, scheme);

pub(crate) async fn connect_directly<R>(request: R) -> Result<WsStream, ConnectError>
where
    R: IntoClientRequest + Unpin,
{
    let request = request.into_client_request().map_err(|e| ConnectError {
        kind: ConnectErrorKind::RequestConstruction,
        source: Some(e.into()),
    })?;
    let addr = request.uri().host_colon_port()?;
    let try_socket = TcpStream::connect(addr).await?;
    let socket = MaybeSocks5Stream::Plain(try_socket);
    Ok(tokio_tungstenite::client_async_tls(request, socket)
        .await?
        .0)
}

pub(crate) async fn connect_via_socks5_proxy(
    ws_req: tokio_tungstenite::tungstenite::handshake::client::Request,
    proxy_addr: &Url,
) -> Result<WsStream, ConnectError> {
    debug!("Using socks5 proxy: {proxy_addr}");
    let proxy_stream = MaybeSocks5Stream::Socks5Stream(
        Socks5Stream::connect(
            proxy_addr.host_and_port()?,
            (
                ws_req
                    .uri()
                    .host()
                    .expect("expected to have uri host")
                    .to_string(),
                ws_req.uri().port_u16().unwrap_or(443),
            ),
        )
        .await?,
    );
    debug!("Connected to socks5 proxy!");
    Ok(tokio_tungstenite::client_async_tls(ws_req, proxy_stream)
        .await?
        .0)
}

pub(crate) async fn connect_via_http_proxy(
    ws_req: tokio_tungstenite::tungstenite::handshake::client::Request,
    proxy_addr: &Url,
) -> Result<WsStream, ConnectError> {
    debug!("Using http proxy: {proxy_addr}");
    let authority = ws_req.uri().host_colon_port()?;
    let proxy_server = proxy_addr.host_colon_port()?;
    let stream = TcpStream::connect(proxy_server).await?;

    let (mut request_sender, conn) = hyper::client::conn::handshake(stream).await?;

    let conn = tokio::spawn(conn.without_shutdown());
    let connect_req = hyper::Request::connect(&authority)
        .body(hyper::Body::empty())
        .map_err(|e| ConnectError {
            kind: ConnectErrorKind::RequestConstruction,
            source: Some(e.into()),
        })?;

    let res = request_sender.send_request(connect_req).await?;

    if !res.status().is_success() {
        return Err(ConnectError {
            source: Some(anyhow::anyhow!(
                "The proxy server returned an error response: status code: {}, body: {:#?}",
                res.status(),
                res.body()
            )),
            kind: ConnectErrorKind::BadResponse,
        });
    }

    let tcp = MaybeSocks5Stream::Plain(
        conn.await
            .map_err(|e| ConnectError {
                kind: ConnectErrorKind::Connection,
                source: Some(e.into()),
            })??
            .io,
    );
    let (ws_stream, _) = tokio_tungstenite::client_async_tls(ws_req, tcp).await?;
    Ok(ws_stream)
}
