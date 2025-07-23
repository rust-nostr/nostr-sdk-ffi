// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;
use std::time::Duration;

use uniffi::Object;

use crate::error::Result;
use crate::relay::RelayOptions;

/// NWC options
#[derive(Clone, Object)]
pub struct NostrWalletConnectOptions {
    inner: nwc::NostrWalletConnectOptions,
}

impl Deref for NostrWalletConnectOptions {
    type Target = nwc::NostrWalletConnectOptions;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[uniffi::export]
impl NostrWalletConnectOptions {
    /// New default NWC options
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            inner: nwc::NostrWalletConnectOptions::new(),
        }
    }

    /// Set NWC requests timeout (default: 10 secs)
    pub fn timeout(&self, timeout: Duration) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.timeout(timeout);
        builder
    }

    /// Set relay options
    pub fn relay(&self, opts: &RelayOptions) -> Result<Self> {
        let mut builder = self.clone();
        builder.inner = builder.inner.relay(opts.deref().clone());
        Ok(builder)
    }
}
