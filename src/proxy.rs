use std::net;
use std::ops::Deref;
use std::sync::Arc;

use nostr_sdk::proxy;
use uniffi::Object;

use crate::error::Result;
use crate::net::SocketAddr;
use crate::protocol::types::RelayUrl;

#[uniffi::export(with_foreign)]
pub trait CustomProxy: Send + Sync {
    /// The callback receives the relay URL and returns the proxy address to use.
    /// Returning null means that the relay should use a direct connection.
    fn custom(&self, relay_url: Arc<RelayUrl>) -> Option<Arc<SocketAddr>>;
}

/// SOCKS5 proxy policy for relay connections.
#[derive(Object)]
pub struct Proxy {
    inner: proxy::Proxy,
}

impl Deref for Proxy {
    type Target = proxy::Proxy;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[uniffi::export]
impl Proxy {
    /// Use a SOCKS5 proxy for all relay connections.
    #[uniffi::constructor]
    pub fn all(addr: &str) -> Result<Self> {
        let addr: net::SocketAddr = addr.parse()?;
        Ok(Self {
            inner: proxy::Proxy::all(addr),
        })
    }

    /// Use a SOCKS5 proxy only for `.onion` relay connections.
    ///
    /// This is a convenience
    /// wrapper around [`Proxy::custom`] for the common Tor SOCKS5 proxy setup.
    #[uniffi::constructor]
    pub fn onion(addr: &str) -> Result<Self> {
        let addr: net::SocketAddr = addr.parse()?;
        Ok(Self {
            inner: proxy::Proxy::onion(addr),
        })
    }

    /// Use a custom SOCKS5 proxy policy.
    ///
    /// Use this when proxy routing depends on application-specific rules, such
    /// as selected domains, user settings, or multiple proxy endpoints.
    #[uniffi::constructor]
    pub fn custom(custom: Arc<dyn CustomProxy>) -> Self {
        Self {
            inner: proxy::Proxy::custom(move |relay_url| {
                let url: nostr::RelayUrl = relay_url.clone();
                let url: Arc<RelayUrl> = Arc::new(url.into());
                let addr: Arc<SocketAddr> = custom.custom(url)?;
                Some(**addr)
            }),
        }
    }
}
