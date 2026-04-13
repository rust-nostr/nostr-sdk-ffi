// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

#[cfg(not(target_arch = "wasm32"))]
use std::net::SocketAddr;
use std::num::NonZeroUsize;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use nostr_sdk::{client, prelude};
use uniffi::{Enum, Object, Record};

use super::Client;
use crate::database::NostrDatabase;
use crate::error::{NostrSdkError, Result};
use crate::gossip::NostrGossip;
use crate::monitor::Monitor;
use crate::policy::{AdmitPolicy, FFI2RustAdmitPolicy};
use crate::protocol::signer::NostrSigner;
#[cfg(not(target_arch = "wasm32"))]
use crate::relay::ConnectionMode;
use crate::relay::RelayLimits;
use crate::transport::websocket::{CustomWebSocketTransport, FFI2RustWebSocketTransport};

/// Max number of relays to use for gossip
#[derive(Record)]
pub struct GossipRelayLimits {
    /// Max number of **read** relays per user (default: 3)
    #[uniffi(default = 3)]
    pub read_relays_per_user: u8,
    /// Max number of **write** relays per user (default: 3)
    #[uniffi(default = 3)]
    pub write_relays_per_user: u8,
    /// Max number of **hint** relays per user (default: 1)
    #[uniffi(default = 1)]
    pub hint_relays_per_user: u8,
    /// Max number of **most used** relays per user (default: 1)
    #[uniffi(default = 1)]
    pub most_used_relays_per_user: u8,
    /// Max number of NIP-17 relays per user (default: 3)
    #[uniffi(default = 3)]
    pub nip17_relays: u8,
}

impl From<GossipRelayLimits> for client::GossipRelayLimits {
    fn from(limits: GossipRelayLimits) -> Self {
        Self {
            read_relays_per_user: limits.read_relays_per_user,
            write_relays_per_user: limits.write_relays_per_user,
            hint_relays_per_user: limits.hint_relays_per_user,
            most_used_relays_per_user: limits.most_used_relays_per_user,
            nip17_relays: limits.nip17_relays,
        }
    }
}

/// Allowed gossip relay types during selection
#[derive(Record)]
pub struct GossipAllowedRelays {
    /// Allow tor onion relays (default: true)
    #[uniffi(default = true)]
    pub onion: bool,
    /// Allow local network relays (default: false)
    #[uniffi(default = false)]
    pub local: bool,
    /// Allow relays without SSL/TLS encryption (default: true)
    #[uniffi(default = true)]
    pub without_tls: bool,
}

impl From<GossipAllowedRelays> for prelude::GossipAllowedRelays {
    fn from(allowed: GossipAllowedRelays) -> Self {
        Self {
            onion: allowed.onion,
            local: allowed.local,
            without_tls: allowed.without_tls,
        }
    }
}

/// Background gossip refresh configuration.
#[derive(Clone, Object)]
pub struct GossipBackgroundRefresh {
    inner: client::GossipBackgroundRefresh,
}

impl Deref for GossipBackgroundRefresh {
    type Target = client::GossipBackgroundRefresh;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<client::GossipBackgroundRefresh> for GossipBackgroundRefresh {
    fn from(inner: client::GossipBackgroundRefresh) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl GossipBackgroundRefresh {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            inner: client::GossipBackgroundRefresh::default(),
        }
    }

    /// Set refresh interval. (default: 5 min)
    pub fn interval(&self, interval: Duration) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.interval(interval);
        builder
    }

    /// Set max public keys refreshed per round and list kind (default: 512)
    pub fn max_public_keys_per_round(&self, max: u64) -> Result<Self> {
        let max: NonZeroUsize = to_non_zero_usize(max, "max_public_keys_per_round")?;
        let mut builder = self.clone();
        builder.inner = builder.inner.max_public_keys_per_round(max);
        Ok(builder)
    }
}

/// Gossip config
#[derive(Clone, Object)]
pub struct GossipConfig {
    inner: client::GossipConfig,
}

impl Deref for GossipConfig {
    type Target = client::GossipConfig;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<client::GossipConfig> for GossipConfig {
    fn from(inner: client::GossipConfig) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl GossipConfig {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            inner: client::GossipConfig::default(),
        }
    }

    /// Max number of gossip relays to use
    pub fn limits(&self, limits: GossipRelayLimits) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.limits(limits.into());
        builder
    }

    /// Allowed relays during gossip selection
    pub fn allowed(&self, allowed: GossipAllowedRelays) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.allowed(allowed.into());
        builder
    }

    /// Timeout for checking if negentropy is supported, when updating gossip data
    pub fn sync_initial_timeout(&self, timeout: Duration) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.sync_initial_timeout(timeout);
        builder
    }

    /// Idle timeout when syncing gossip data
    pub fn sync_idle_timeout(&self, timeout: Duration) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.sync_idle_timeout(timeout);
        builder
    }

    /// Fetch timeout when updating gossip data (fallback of the sync)
    pub fn fetch_timeout(&self, timeout: Duration) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.fetch_timeout(timeout);
        builder
    }

    /// REQ chunks when fetching gossip data
    pub fn fetch_chunks(&self, chunks: u64) -> Result<Self> {
        let chunks: usize = to_usize(chunks, "fetch_chunks")?;
        let mut builder = self.clone();
        builder.inner = builder.inner.fetch_chunks(chunks);
        Ok(builder)
    }

    /// Configure background refresh
    pub fn background_refresh(&self, config: &GossipBackgroundRefresh) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.background_refresh(**config);
        builder
    }

    /// Disable background refresh
    pub fn no_background_refresh(&self) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.no_background_refresh();
        builder
    }
}

/// Connection target
#[cfg(not(target_arch = "wasm32"))]
#[derive(Enum)]
pub enum ConnectionTarget {
    /// Use proxy for all relays
    All,
    /// Use proxy only for `.onion` relays
    Onion,
}

#[cfg(not(target_arch = "wasm32"))]
impl From<ConnectionTarget> for client::ConnectionTarget {
    fn from(value: ConnectionTarget) -> Self {
        match value {
            ConnectionTarget::All => Self::All,
            ConnectionTarget::Onion => Self::Onion,
        }
    }
}

/// Connection
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct Connection {
    inner: client::Connection,
}

#[cfg(not(target_arch = "wasm32"))]
impl Deref for Connection {
    type Target = client::Connection;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[uniffi::export]
impl Connection {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            inner: client::Connection::new(),
        }
    }

    /// Set connection mode (default: direct)
    pub fn mode(&self, mode: ConnectionMode) -> Result<Self> {
        let mode: prelude::ConnectionMode = mode.try_into()?;
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

    /// Set direct connection
    pub fn direct(&self) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.direct();
        builder
    }
}

/// Put relays to sleep when idle.
#[derive(Enum)]
pub enum SleepWhenIdle {
    /// Disabled
    Disabled,
    /// Enabled for all relays
    Enabled {
        /// Idle timeout
        ///
        /// After how much time of inactivity put the relay to sleep.
        timeout: Duration,
    },
}

impl From<SleepWhenIdle> for client::SleepWhenIdle {
    fn from(value: SleepWhenIdle) -> Self {
        match value {
            SleepWhenIdle::Disabled => Self::Disabled,
            SleepWhenIdle::Enabled { timeout } => Self::Enabled { timeout },
        }
    }
}

#[derive(Clone, Default, Object)]
pub struct ClientBuilder {
    inner: client::ClientBuilder,
}

impl From<client::ClientBuilder> for ClientBuilder {
    fn from(inner: client::ClientBuilder) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl ClientBuilder {
    /// New client builder
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn signer(&self, signer: &NostrSigner) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.signer(signer.deref().clone());
        builder
    }

    pub fn database(&self, database: &NostrDatabase) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.database(database.deref().clone());
        builder
    }

    /// Set a gossip store
    pub fn gossip(&self, gossip: &NostrGossip) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.gossip(gossip.deref().clone());
        builder
    }

    /// Set a custom WebSocket transport
    pub fn websocket_transport(&self, transport: Arc<dyn CustomWebSocketTransport>) -> Self {
        let mut builder = self.clone();
        let intermediate = FFI2RustWebSocketTransport { inner: transport };
        builder.inner = builder.inner.websocket_transport(intermediate);
        builder
    }

    /// Set an admission policy
    pub fn admit_policy(&self, policy: Arc<dyn AdmitPolicy>) -> Self {
        let mut builder = self.clone();
        let intermediate = FFI2RustAdmitPolicy { inner: policy };
        builder.inner = builder.inner.admit_policy(intermediate);
        builder
    }

    /// Set monitor
    #[inline]
    pub fn monitor(&self, monitor: &Monitor) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.monitor(monitor.deref().clone());
        builder
    }

    /// Max relays allowed in the pool (default: None)
    ///
    /// `None` means no limit.
    pub fn max_relays(&self, num: Option<u64>) -> Result<Self> {
        let mut builder = self.clone();
        let num: Option<NonZeroUsize> = match num {
            Some(num) => Some(to_non_zero_usize(num, "max_relays")?),
            None => None,
        };
        builder.inner = builder.inner.max_relays(num);
        Ok(builder)
    }

    /// Auto authenticates to relays (default: true)
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/42.md>
    pub fn automatic_authentication(&self, enabled: bool) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.automatic_authentication(enabled);
        builder
    }

    /// Connection timeout (default: 15 sec)
    ///
    /// This is the default timeout use when attempting to establish a connection with the relay.
    pub fn connect_timeout(&self, timeout: Duration) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.connect_timeout(timeout);
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

    /// Set sleep when idle config
    pub fn sleep_when_idle(&self, config: SleepWhenIdle) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.sleep_when_idle(config.into());
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

    /// Notification channel size (default: 4096)
    pub fn notification_channel_size(&self, size: u64) -> Result<Self> {
        let size: NonZeroUsize = to_non_zero_usize(size, "notification_channel_size")?;
        let mut builder = self.clone();
        builder.inner = builder.inner.notification_channel_size(size);
        Ok(builder)
    }

    /// Gossip config
    pub fn gossip_config(&self, config: &GossipConfig) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.gossip_config(config.deref().clone());
        builder
    }

    /// Build [`Client`]
    pub fn build(&self) -> Client {
        let inner = self.inner.clone();
        inner.build().into()
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[uniffi::export]
impl ClientBuilder {
    /// Connection mode and target
    pub fn connection(&self, connection: &Connection) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.connection(connection.deref().clone());
        builder
    }
}

fn to_usize(value: u64, field: &str) -> Result<usize> {
    usize::try_from(value)
        .map_err(|_| NostrSdkError::Generic(format!("{field} is too large for this platform")))
}

fn to_non_zero_usize(value: u64, field: &str) -> Result<NonZeroUsize> {
    let value: usize = to_usize(value, field)?;
    NonZeroUsize::new(value)
        .ok_or_else(|| NostrSdkError::Generic(format!("{field} must be greater than 0")))
}
