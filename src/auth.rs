use std::borrow::Cow;

use hyper::{header::HeaderName, http::HeaderValue};

/// Options for authentication
#[derive(Debug, Clone)]
pub struct AuthOptions<'a> {
    /// Endpoint of the service
    pub(crate) endpoint: Cow<'a, str>,
    /// Authentication token
    pub(crate) token: Option<Cow<'a, str>>,
    /// Azure Subscription Key for authentication.
    pub(crate) key: Option<Cow<'a, str>>,
    /// Additional headers
    pub(crate) headers: Cow<'a, [(HeaderName, HeaderValue)]>,
    /// Proxy server to use. Only http and socks5 proxy are supported by now.
    pub(crate) proxy: Option<Cow<'a, str>>,
}

impl<'a> AuthOptions<'a> {
    /// Endpoint of the service, typically a URL with `wss` protocol
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Endpoint of the service, typically a URL with `wss` protocol
    pub fn endpoint_mut(&mut self) -> &mut Cow<'a, str> {
        &mut self.endpoint
    }

    /// Authentication token
    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    /// Authentication token
    pub fn token_mut(&mut self) -> &mut Option<Cow<'a, str>> {
        &mut self.token
    }

    /// Azure Subscription Key for authentication.
    pub fn key(&self) -> Option<&str> {
        self.key.as_deref()
    }

    /// Azure Subscription Key for authentication.
    pub fn key_mut(&mut self) -> &mut Option<Cow<'a, str>> {
        &mut self.key
    }

    /// Additional request headers
    pub fn headers(&self) -> &[(HeaderName, HeaderValue)] {
        &self.headers
    }

    /// Additional request headers
    pub fn headers_mut(&mut self) -> &mut Cow<'a, [(HeaderName, HeaderValue)]> {
        &mut self.headers
    }

    /// Proxy server to use. Only http and socks5 proxy are supported by now.
    pub fn proxy(&self) -> Option<&str> {
        self.proxy.as_deref()
    }

    /// Proxy server to use. Only http and socks5 proxy are supported by now.
    pub fn proxy_mut(&mut self) -> &mut Option<Cow<'a, str>> {
        &mut self.proxy
    }

    /// Create a builder for `AuthOptions`
    pub fn builder(endpoint: impl Into<Cow<'a, str>>) -> AuthOptionsBuilder<'a> {
        AuthOptionsBuilder::new(endpoint)
    }
}

/// Builder for `AuthOptions`
pub struct AuthOptionsBuilder<'a> {
    endpoint: Cow<'a, str>,
    token: Option<Cow<'a, str>>,
    key: Option<Cow<'a, str>>,
    headers: Cow<'a, [(HeaderName, HeaderValue)]>,
    proxy: Option<Cow<'a, str>>,
}

impl<'a> AuthOptionsBuilder<'a> {
    /// Create a new builder
    ///
    /// # Arguments
    ///
    /// * `endpoint` - Endpoint of the service, typically a URL with `wss` protocol
    pub fn new(endpoint: impl Into<Cow<'a, str>>) -> Self {
        Self {
            endpoint: endpoint.into(),
            token: Default::default(),
            key: Default::default(),
            headers: Default::default(),
            proxy: Default::default(),
        }
    }

    /// Authentication token
    pub fn token(mut self, token: impl Into<Cow<'a, str>>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Authentication token
    pub fn optional_token(mut self, token: Option<impl Into<Cow<'a, str>>>) -> Self {
        self.token = token.map(Into::into);
        self
    }

    /// Azure Subscription Key for authentication.
    pub fn key(mut self, key: impl Into<Cow<'a, str>>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// Azure Subscription Key for authentication.
    pub fn optional_key(mut self, key: Option<impl Into<Cow<'a, str>>>) -> Self {
        self.key = key.map(Into::into);
        self
    }

    /// Additional request headers
    pub fn headers(mut self, headers: impl Into<Cow<'a, [(HeaderName, HeaderValue)]>>) -> Self {
        self.headers = headers.into();
        self
    }

    /// Proxy server to use. Only http and socks5 proxy are supported by now.
    pub fn proxy(mut self, proxy: impl Into<Cow<'a, str>>) -> Self {
        self.proxy = Some(proxy.into());
        self
    }

    /// Proxy server to use. Only http and socks5 proxy are supported by now.
    pub fn optional_proxy(mut self, proxy: Option<impl Into<Cow<'a, str>>>) -> Self {
        self.proxy = proxy.map(Into::into);
        self
    }

    /// Build `AuthOptions`
    pub fn build(self) -> AuthOptions<'a> {
        AuthOptions {
            endpoint: self.endpoint,
            token: self.token,
            key: self.key,
            headers: self.headers,
            proxy: self.proxy,
        }
    }
}
