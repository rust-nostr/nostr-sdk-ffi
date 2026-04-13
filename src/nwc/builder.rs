use std::ops::Deref;
use std::time::Duration;

use nwc::builder;
use uniffi::Object;

use super::NostrWalletConnect;
use crate::monitor::Monitor;
use crate::protocol::nips::nip47::NostrWalletConnectUri;
use crate::relay::RelayOptions;

#[derive(Clone, Object)]
pub struct NostrWalletConnectBuilder {
    inner: builder::NostrWalletConnectBuilder,
}

impl Deref for NostrWalletConnectBuilder {
    type Target = builder::NostrWalletConnectBuilder;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[uniffi::export]
impl NostrWalletConnectBuilder {
    /// Construct a new Nostr Wallet Connect client builder
    #[uniffi::constructor]
    pub fn new(uri: &NostrWalletConnectUri) -> Self {
        Self {
            inner: builder::NostrWalletConnectBuilder::new(uri.deref().clone()),
        }
    }

    /// Set NWC requests timeout (default: 10 secs)
    pub fn timeout(&self, timeout: Duration) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.timeout(timeout);
        builder
    }

    /// Set the relay monitor
    pub fn monitor(&self, monitor: &Monitor) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.monitor(monitor.deref().clone());
        builder
    }

    /// Set relay options
    pub fn relay(&self, opts: &RelayOptions) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.relay(opts.deref().clone());
        builder
    }

    #[inline]
    pub fn build(&self) -> NostrWalletConnect {
        self.inner.clone().build().into()
    }
}
