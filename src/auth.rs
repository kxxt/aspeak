use std::borrow::Cow;

use hyper::{header::HeaderName, http::HeaderValue};

/// Options for authentication
#[derive(Debug, Clone)]
pub struct AuthOptions<'a> {
    /// Endpoint of the service
    pub(crate) endpoint: Cow<'a, str>,
    /// Authentication token
    pub(crate) token: Option<Cow<'a, str>>,
    /// Azure Subscription Key for authentication. It currently doesn't work.
    pub(crate) key: Option<Cow<'a, str>>,
    /// Additional headers
    pub(crate) headers: Cow<'a, [(HeaderName, HeaderValue)]>,
    /// Proxy server to use. Only http and socks5 proxy are supported by now.
    pub(crate) proxy: Option<Cow<'a, str>>,
}

impl<'a> AuthOptions<'a> {
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub fn key(&self) -> Option<&str> {
        self.key.as_deref()
    }

    pub fn headers(&self) -> &[(HeaderName, HeaderValue)] {
        &self.headers
    }

    pub fn proxy(&self) -> Option<&str> {
        self.proxy.as_deref()
    }

    pub fn builder(endpoint: impl Into<Cow<'a, str>>) -> AuthOptionsBuilder<'a> {
        AuthOptionsBuilder::new(endpoint)
    }
}

pub struct AuthOptionsBuilder<'a> {
    endpoint: Cow<'a, str>,
    token: Option<Cow<'a, str>>,
    key: Option<Cow<'a, str>>,
    headers: Cow<'a, [(HeaderName, HeaderValue)]>,
    proxy: Option<Cow<'a, str>>,
}

impl<'a> AuthOptionsBuilder<'a> {
    pub fn new(endpoint: impl Into<Cow<'a, str>>) -> Self {
        Self {
            endpoint: endpoint.into(),
            token: Default::default(),
            key: Default::default(),
            headers: Default::default(),
            proxy: Default::default(),
        }
    }

    pub fn token(mut self, token: impl Into<Cow<'a, str>>) -> Self {
        self.token = Some(token.into());
        self
    }

    pub fn key(mut self, key: impl Into<Cow<'a, str>>) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn headers(mut self, headers: impl Into<Cow<'a, [(HeaderName, HeaderValue)]>>) -> Self {
        self.headers = headers.into();
        self
    }

    pub fn proxy(mut self, proxy: impl Into<Cow<'a, str>>) -> Self {
        self.proxy = Some(proxy.into());
        self
    }

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
