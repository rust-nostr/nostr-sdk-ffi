// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use nostr_connect::client;
use uniffi::Object;

use crate::error::Result;
use crate::protocol::key::Keys;
use crate::protocol::nips::nip46::NostrConnectUri;
use crate::protocol::signer::{AsyncNostrSigner, export_async_nostr_signer};
use crate::protocol::types::RelayUrl;
use crate::relay::RelayOptions;

#[derive(Object)]
pub struct NostrConnect {
    inner: client::NostrConnect,
}

impl Deref for NostrConnect {
    type Target = client::NostrConnect;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<client::NostrConnect> for NostrConnect {
    fn from(inner: client::NostrConnect) -> Self {
        Self { inner }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl NostrConnect {
    /// Construct Nostr Connect client
    #[uniffi::constructor]
    pub fn new(
        uri: &NostrConnectUri,
        app_keys: &Keys,
        timeout: Duration,
        opts: Option<Arc<RelayOptions>>,
    ) -> Result<Self> {
        Ok(Self {
            inner: client::NostrConnect::new(
                uri.deref().clone(),
                app_keys.deref().clone(),
                timeout,
                opts.map(|k| k.as_ref().deref().clone()),
            )?,
        })
    }

    /// Get signer relays
    pub fn relays(&self) -> Vec<Arc<RelayUrl>> {
        self.inner
            .relays()
            .iter()
            .cloned()
            .map(|u| Arc::new(u.into()))
            .collect()
    }

    /// Get `bunker` URI
    pub async fn bunker_uri(&self) -> Result<NostrConnectUri> {
        Ok(self.inner.bunker_uri().await?.into())
    }
}

export_async_nostr_signer!(NostrConnect, |signer| &signer.inner);
