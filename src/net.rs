use std::fmt;
use std::ops::Deref;

use uniffi::Object;

use crate::error::Result;

/// Socket address
#[derive(Debug, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Display, Eq, Hash)]
pub struct SocketAddr {
    inner: std::net::SocketAddr,
}

impl Deref for SocketAddr {
    type Target = std::net::SocketAddr;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<std::net::SocketAddr> for SocketAddr {
    fn from(inner: std::net::SocketAddr) -> Self {
        Self { inner }
    }
}

impl fmt::Display for SocketAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[uniffi::export]
impl SocketAddr {
    /// Parse a socket address (i.e., 192.168.1.100:80)
    #[uniffi::constructor]
    pub fn parse(addr: &str) -> Result<Self> {
        Ok(Self {
            inner: addr.parse()?,
        })
    }

    /// Get the IP address
    #[inline]
    pub fn ip(&self) -> String {
        self.inner.ip().to_string()
    }

    /// Get the port
    #[inline]
    pub fn port(&self) -> u16 {
        self.inner.port()
    }
}
