// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::fmt;
use std::ops::Deref;

use uniffi::Object;

use crate::error::Result;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Display, Eq, Hash)]
pub struct RelayUrl {
    inner: nostr::RelayUrl,
}

impl fmt::Display for RelayUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Deref for RelayUrl {
    type Target = nostr::RelayUrl;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<nostr::RelayUrl> for RelayUrl {
    fn from(inner: nostr::RelayUrl) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl RelayUrl {
    /// Parse a relay URL
    #[uniffi::constructor]
    pub fn parse(url: &str) -> Result<RelayUrl> {
        Ok(Self {
            inner: nostr::RelayUrl::parse(url)?,
        })
    }

    /// Check if the host is a local network address.
    ///
    /// IPv4 address ranges:
    /// - `127.0.0.0/8`
    /// - `10.0.0.0/8`
    /// - `172.16.0.0/12`
    /// - `192.168.0.0/16`
    ///
    /// IPv6 address ranges:
    /// - `::1`
    #[inline]
    pub fn is_local_addr(&self) -> bool {
        self.inner.is_local_addr()
    }

    /// Check if the URL is a hidden onion service address
    #[inline]
    pub fn is_onion(&self) -> bool {
        self.inner.is_onion()
    }
}
