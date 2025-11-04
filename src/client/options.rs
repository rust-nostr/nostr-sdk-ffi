// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::net::SocketAddr;
use std::ops::Deref;
use std::time::Duration;

use nostr_sdk::client::options;
use nostr_sdk::pool;
use uniffi::{Enum, Object, Record};

use crate::error::Result;
use crate::relay::{ConnectionMode, RelayLimits};

/// Max number of relays to use for gossip
#[derive(Record)]
pub struct GossipRelayLimits {
    /// Max number of **read** relays per user (default: 3)
    pub read_relays_per_user: u64,
    /// Max number of **write** relays per user (default: 3)
    pub write_relays_per_user: u64,
    /// Max number of **hint** relays per user (default: 1)
    pub hint_relays_per_user: u64,
    /// Max number of **most used** relays per user (default: 1)
    pub most_used_relays_per_user: u64,
    /// Max number of NIP-17 relays per user (default: 3)
    pub nip17_relays: u64,
}

impl From<GossipRelayLimits> for options::GossipRelayLimits {
    fn from(limits: GossipRelayLimits) -> Self {
        Self {
            read_relays_per_user: limits.read_relays_per_user as usize,
            write_relays_per_user: limits.write_relays_per_user as usize,
            hint_relays_per_user: limits.hint_relays_per_user as usize,
            most_used_relays_per_user: limits.most_used_relays_per_user as usize,
            nip17_relays: limits.nip17_relays as usize,
        }
    }
}

/// Gossip options
#[derive(Record)]
pub struct GossipOptions {
    /// Max number of relays to use
    pub limits: GossipRelayLimits,
}

impl From<GossipOptions> for options::GossipOptions {
    fn from(opts: GossipOptions) -> Self {
        Self {
            limits: opts.limits.into(),
        }
    }
}

/// Nostr client options
#[derive(Clone, Object)]
pub struct ClientOptions {
    inner: options::ClientOptions,
}

impl Deref for ClientOptions {
    type Target = options::ClientOptions;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<options::ClientOptions> for ClientOptions {
    fn from(inner: options::ClientOptions) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl ClientOptions {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            inner: options::ClientOptions::new(),
        }
    }

    /// Automatically start connection with relays (default: false)
    ///
    /// When set to `true`, there isn't the need of calling the connect methods.
    pub fn autoconnect(&self, val: bool) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.autoconnect(val);
        builder
    }

    /// Auto authenticate to relays (default: true)
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/42.md>
    pub fn automatic_authentication(&self, enabled: bool) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.automatic_authentication(enabled);
        builder
    }

    /// Connection
    pub fn connection(&self, connection: &Connection) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.connection(connection.deref().clone());
        builder
    }

    /// Set custom relay limits
    pub fn relay_limits(&self, limits: &RelayLimits) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.relay_limits(limits.deref().clone());
        builder
    }

    /// Set max latency (default: None)
    ///
    /// Relays with an avg. latency greater that this value will be skipped.
    pub fn max_avg_latency(&self, max: Duration) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.max_avg_latency(max);
        builder
    }

    /// Verify that received events belong to a subscription and match the filter.
    pub fn verify_subscriptions(&self, enable: bool) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.verify_subscriptions(enable);
        builder
    }

    /// If true, ban a relay when it sends an event that doesn't match the subscription filter.
    pub fn ban_relay_on_mismatch(&self, enable: bool) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.ban_relay_on_mismatch(enable);
        builder
    }

    /// Gossip options
    pub fn gossip(&self, opts: GossipOptions) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.gossip(opts.into());
        builder
    }
}

/// Connection target
#[derive(Enum)]
pub enum ConnectionTarget {
    /// Use proxy for all relays
    All,
    /// Use proxy only for `.onion` relays
    Onion,
}

impl From<ConnectionTarget> for options::ConnectionTarget {
    fn from(value: ConnectionTarget) -> Self {
        match value {
            ConnectionTarget::All => Self::All,
            ConnectionTarget::Onion => Self::Onion,
        }
    }
}

/// Connection
#[derive(Debug, Clone, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct Connection {
    inner: options::Connection,
}

impl Deref for Connection {
    type Target = options::Connection;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[uniffi::export]
impl Connection {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            inner: options::Connection::default(),
        }
    }

    /// Set connection mode (default: direct)
    pub fn mode(&self, mode: ConnectionMode) -> Result<Self> {
        let mode: pool::ConnectionMode = mode.try_into()?;
        let mut builder = self.clone();
        builder.inner = builder.inner.mode(mode);
        Ok(builder)
    }

    /// Set connection target (default: all)
    pub fn target(&self, target: ConnectionTarget) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.target(target.into());
        builder
    }

    /// Set proxy (ex. `127.0.0.1:9050`)
    pub fn addr(&self, addr: &str) -> Result<Self> {
        let mut builder = self.clone();
        let addr: SocketAddr = addr.parse()?;
        builder.inner = builder.inner.proxy(addr);
        Ok(builder)
    }
}

#[cfg(feature = "tor")]
#[uniffi::export]
impl Connection {
    /// Use the embedded tor client
    ///
    /// This doesn't work on `android` and/or `ios` targets.
    /// Use [`Connection::embedded_tor_with_path`] instead.
    pub fn embedded_tor(&self) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.embedded_tor();
        builder
    }

    /// Use the embedded tor client
    ///
    /// Specify a path where to store the tor data
    pub fn embedded_tor_with_path(&self, data_path: String) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.embedded_tor_with_path(data_path);
        builder
    }
}
