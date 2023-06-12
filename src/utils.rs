use reqwest::{ClientBuilder, Proxy};

pub(crate) trait ClientBuilderExt {
    fn optional_proxy(self, proxy: Option<Proxy>) -> Self;
}

impl ClientBuilderExt for ClientBuilder {
    fn optional_proxy(self, proxy: Option<Proxy>) -> Self {
        if let Some(proxy) = proxy {
            self.proxy(proxy)
        } else {
            self
        }
    }
}

#[cfg(feature = "rest-synthesizer")]
pub(crate) fn transpose_tuple_option_result<T, K, E>(
    x: Option<(T, Result<K, E>)>,
) -> Result<Option<(T, K)>, E> {
    match x {
        Some((t, Ok(k))) => Ok(Some((t, k))),
        Some((_, Err(e))) => Err(e),
        None => Ok(None),
    }
}
