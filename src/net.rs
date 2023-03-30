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

use fast_socks5::client::Socks5Stream;
use tokio_tungstenite::{tungstenite::client::IntoClientRequest, MaybeTlsStream, WebSocketStream};

use crate::error::{AspeakError, Result};

type TungsteniteError = tokio_tungstenite::tungstenite::Error;
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
    fn host_colon_port(&self) -> Result<String>;
}

impl UriExt for Uri {
    fn host_colon_port(&self) -> Result<String> {
        let port = match (self.port_u16(), self.scheme_str()) {
            (Some(port), _) => port,
            (None, Some("wss") | Some("https")) => 443,
            (None, Some("ws") | Some("http")) => 80,
            x => {
                return Err(AspeakError::GeneralConnectionError(format!(
                    "No scheme or unsupported scheme: {:?}",
                    x.1
                )))
            }
        };
        let host = self.host().ok_or_else(|| {
            AspeakError::GeneralConnectionError(format!("No host in uri: {}", self))
        })?;
        Ok(format!("{}:{}", host, port))
    }
}

impl UriExt for Url {
    fn host_colon_port(&self) -> Result<String> {
        let port = match (self.port(), self.scheme()) {
            (Some(port), _) => port,
            (None, "wss" | "https") => 443,
            (None, "ws" | "http") => 80,
            x => {
                return Err(AspeakError::GeneralConnectionError(format!(
                    "No scheme or unsupported scheme: {:?}",
                    x.1
                )))
            }
        };
        let host = self.host_str().ok_or_else(|| {
            AspeakError::GeneralConnectionError(format!("No host in uri: {}", self))
        })?;
        Ok(format!("{}:{}", host, port))
    }
}

pub(crate) async fn connect_directly<R>(
    request: R,
) -> Result<WsStream>
where
    R: IntoClientRequest + Unpin,
{
    let request = request.into_client_request()?;
    let addr = request.uri().host_colon_port()?;
    let try_socket = TcpStream::connect(addr).await;
    let socket = MaybeSocks5Stream::Plain(try_socket.map_err(TungsteniteError::Io)?);
    Ok(tokio_tungstenite::client_async_tls(request, socket)
        .await?
        .0)
}

pub(crate) async fn connect_via_socks5_proxy(
    ws_req: tokio_tungstenite::tungstenite::handshake::client::Request,
    proxy_addr: &Url,
) -> Result<WsStream> {
    debug!("Using socks5 proxy: {proxy_addr}");
    let proxy_stream = MaybeSocks5Stream::Socks5Stream(
        Socks5Stream::connect(
            proxy_addr.host_colon_port()?,
            ws_req
                .uri()
                .host()
                .expect("expected to have uri host")
                .to_string(),
            ws_req.uri().port_u16().unwrap_or(443),
            fast_socks5::client::Config::default(),
        )
        .await
        .map_err(|e| {
            AspeakError::GeneralConnectionError(format!(
                "Failed to connect to socks5 proxy. Details: {e}"
            ))
        })?,
    );
    debug!("Connected to socks5 proxy!");
    Ok(tokio_tungstenite::client_async_tls(ws_req, proxy_stream)
        .await?
        .0)
}

pub(crate) async fn connect_via_http_proxy(
    ws_req: tokio_tungstenite::tungstenite::handshake::client::Request,
    proxy_addr: &Url,
) -> Result<WsStream> {
    debug!("Using http proxy: {proxy_addr}");
    let authority = ws_req.uri().host_colon_port()?;
    let proxy_server = proxy_addr.host_colon_port()?;
    let stream = TcpStream::connect(proxy_server).await?;

    let (mut request_sender, conn) = hyper::client::conn::handshake(stream).await.map_err(|e| {
        AspeakError::GeneralConnectionError(format!(
            "Failed to handshake with proxy server! Details: {e}"
        ))
    })?;

    let conn = tokio::spawn(conn.without_shutdown());
    let connect_req = hyper::Request::connect(&authority)
        .body(hyper::Body::empty())
        .expect("expected to make connect request");

    let res = request_sender
        .send_request(connect_req)
        .await
        .map_err(|e| {
            AspeakError::GeneralConnectionError(format!(
                "Failed to send request to proxy server. Details: {e}"
            ))
        })?;

    if !res.status().is_success() {
        return Err(AspeakError::GeneralConnectionError(format!(
            "The proxy server returned an error response: status code: {}, body: {:#?}",
            res.status(),
            res.body()
        )));
    }

    let tcp = MaybeSocks5Stream::Plain(conn.await.unwrap().unwrap().io);
    let (ws_stream, _) = tokio_tungstenite::client_async_tls(ws_req, tcp).await?;
    Ok(ws_stream)
}
