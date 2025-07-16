// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use nostr::nips::nip05;
use uniffi::Object;

use crate::error::Result;
use crate::protocol::key::PublicKey;
use crate::protocol::types::RelayUrl;

/// NIP-05 address
#[derive(Debug, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Display, Eq, Hash)]
pub struct Nip05Address {
    inner: nip05::Nip05Address,
}

impl fmt::Display for Nip05Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Deref for Nip05Address {
    type Target = nip05::Nip05Address;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[uniffi::export]
impl Nip05Address {
    /// Parse a NIP-05 address (i.e., `yuki@yukikishimoto.com`).
    #[uniffi::constructor]
    pub fn parse(address: &str) -> Result<Self> {
        Ok(Self {
            inner: nip05::Nip05Address::parse(address)?,
        })
    }

    /// Get the name value
    #[inline]
    pub fn name(&self) -> String {
        self.inner.name().to_string()
    }

    /// Get the domain value
    #[inline]
    pub fn domain(&self) -> String {
        self.inner.domain().to_string()
    }

    /// Get url for NIP05 address
    ///
    /// This can be used to make a `GET` HTTP request and get the NIP-05 JSON.
    #[inline]
    pub fn url(&self) -> String {
        self.inner.url().to_string()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct Nip05Profile {
    inner: nip05::Nip05Profile,
}

impl From<nip05::Nip05Profile> for Nip05Profile {
    fn from(inner: nip05::Nip05Profile) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl Nip05Profile {
    /// Extract a NIP-05 profile from raw JSON
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/05.md>
    #[uniffi::constructor]
    pub fn from_json(address: &Nip05Address, json: &str) -> Result<Self> {
        Ok(Self {
            inner: nip05::Nip05Profile::from_raw_json(address.deref(), json)?,
        })
    }

    pub fn public_key(&self) -> PublicKey {
        self.inner.public_key.into()
    }

    /// Get relays
    pub fn relays(&self) -> Vec<Arc<RelayUrl>> {
        self.inner
            .relays
            .iter()
            .cloned()
            .map(|u| Arc::new(u.into()))
            .collect()
    }

    /// Get NIP46 relays
    pub fn nip46(&self) -> Vec<Arc<RelayUrl>> {
        self.inner
            .nip46
            .iter()
            .cloned()
            .map(|u| Arc::new(u.into()))
            .collect()
    }
}

/// Verify a NIP-05 from JSON
#[uniffi::export]
pub fn nip05_verify_from_json(
    public_key: &PublicKey,
    address: &Nip05Address,
    json: &str,
) -> Result<bool> {
    Ok(nip05::verify_from_raw_json(
        public_key.deref(),
        address.deref(),
        json,
    )?)
}
