// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use nostr::SubscriptionId;
use nostr_sdk::relay;
use uniffi::{Object, Record};

pub mod capabilities;
pub mod limits;
pub mod options;
pub mod stats;
pub mod status;

pub use self::limits::RelayLimits;
use self::options::SyncOptions;
pub use self::options::{ConnectionMode, RelayOptions, ReqExitPolicy};
pub use self::stats::RelayConnectionStats;
pub use self::status::RelayStatus;
use crate::database::events::Events;
use crate::error::Result;
use crate::negentropy::NegentropyItem;
use crate::protocol::event::{Event, EventId};
use crate::protocol::filter::Filter;
use crate::protocol::message::ClientMessage;
use crate::protocol::types::RelayUrl;
use crate::relay::options::SubscribeAutoCloseOptions;

/// Relay sync summary
#[derive(Record)]
pub struct RelaySyncSummary {
    /// The IDs that were stored locally
    pub local: Vec<Arc<EventId>>,
    /// The IDs that were missing locally (stored on relay)
    pub remote: Vec<Arc<EventId>>,
    /// Events that are **successfully** sent to relays during reconciliation
    pub sent: Vec<Arc<EventId>>,
    /// Event that are **successfully** received from relay
    pub received: Vec<Arc<EventId>>,
    /// Events that failed to send to relays during reconciliation
    pub send_failures: HashMap<Arc<EventId>, String>,
}

impl From<relay::SyncSummary> for RelaySyncSummary {
    fn from(value: relay::SyncSummary) -> Self {
        Self {
            local: value
                .local
                .into_iter()
                .map(|e| Arc::new(e.into()))
                .collect(),
            remote: value
                .remote
                .into_iter()
                .map(|e| Arc::new(e.into()))
                .collect(),
            sent: value.sent.into_iter().map(|e| Arc::new(e.into())).collect(),
            received: value
                .received
                .into_iter()
                .map(|e| Arc::new(e.into()))
                .collect(),
            send_failures: value
                .send_failures
                .into_iter()
                .map(|(id, error)| (Arc::new(id.into()), error))
                .collect(),
        }
    }
}

#[derive(Object)]
pub struct Relay {
    inner: relay::Relay,
}

impl From<relay::Relay> for Relay {
    fn from(inner: relay::Relay) -> Self {
        Self { inner }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl Relay {
    /// Get relay url
    pub fn url(&self) -> RelayUrl {
        self.inner.url().clone().into()
    }

    /// Get connection mode
    pub fn connection_mode(&self) -> ConnectionMode {
        self.inner.connection_mode().clone().into()
    }

    /// Get status
    pub fn status(&self) -> RelayStatus {
        self.inner.status().into()
    }

    pub async fn subscriptions(&self) -> HashMap<String, Vec<Arc<Filter>>> {
        self.inner
            .subscriptions()
            .await
            .into_iter()
            .map(|(id, filters)| {
                (
                    id.to_string(),
                    filters.into_iter().map(|f| Arc::new(f.into())).collect(),
                )
            })
            .collect()
    }

    /// Get filters by subscription ID
    pub async fn subscription(&self, id: String) -> Option<Vec<Arc<Filter>>> {
        let id = SubscriptionId::new(id);
        self.inner
            .subscription(&id)
            .await
            .map(|filters| filters.into_iter().map(|f| Arc::new(f.into())).collect())
    }

    pub fn opts(&self) -> RelayOptions {
        self.inner.opts().clone().into()
    }

    pub fn stats(&self) -> RelayConnectionStats {
        self.inner.stats().clone().into()
    }

    // TODO: add notifications

    /// Connect to the relay
    ///
    /// # Overview
    ///
    /// If the relay’s status is not [`RelayStatus::Initialized`] or [`RelayStatus::Terminated`],
    /// this method returns immediately without doing anything.
    /// Otherwise, the connection task will be spawned, which will attempt to connect to relay.
    ///
    /// This method returns immediately and doesn't provide any information on if the connection was successful or not.
    ///
    /// # Automatic reconnection
    ///
    /// By default, in case of disconnection, the connection task will automatically attempt to reconnect.
    /// This behavior can be disabled by changing [`RelayOptions::reconnect`] option.
    pub fn connect(&self) {
        self.inner.connect()
    }

    /// Try to establish a connection with the relay.
    ///
    /// # Overview
    ///
    /// If the relay’s status is not [`RelayStatus::Initialized`] or [`RelayStatus::Terminated`],
    /// this method returns immediately without doing anything.
    /// Otherwise, attempts to establish a connection without spawning the connection task if it fails.
    /// This means that if the connection fails, no automatic retries are scheduled.
    /// Use [`Relay::connect`] if you want to immediately spawn a connection task,
    /// regardless of whether the initial connection succeeds.
    ///
    /// Returns an error if the connection fails.
    ///
    /// # Automatic reconnection
    ///
    /// By default, in case of disconnection (after a first successful connection),
    /// the connection task will automatically attempt to reconnect.
    /// This behavior can be disabled by changing [`RelayOptions::reconnect`] option.
    pub async fn try_connect(&self, timeout: Duration) -> Result<()> {
        Ok(self.inner.try_connect().timeout(timeout).await?)
    }

    /// Disconnect from relay and set status to `Terminated`
    pub fn disconnect(&self) {
        self.inner.disconnect()
    }

    /// Ban relay and set status to `Banned`.
    ///
    /// A banned relay can't reconnect again.
    #[inline]
    pub fn ban(&self) {
        self.inner.ban()
    }

    /// Send msg to relay
    pub async fn send_msg(&self, msg: &ClientMessage) -> Result<()> {
        Ok(self.inner.send_msg(msg.deref().clone()).await?)
    }

    /// Send event and wait for `OK` relay msg
    pub async fn send_event(&self, event: &Event) -> Result<Arc<EventId>> {
        Ok(Arc::new(self.inner.send_event(event.deref()).await?.into()))
    }

    /// Subscribe to filters
    #[uniffi::method(default(id = None, close_on = None))]
    pub async fn subscribe(
        &self,
        filter: &Filter,
        id: Option<String>,
        close_on: Option<Arc<SubscribeAutoCloseOptions>>,
    ) -> Result<String> {
        let mut builder = self.inner.subscribe(filter.deref().clone());

        if let Some(id) = id {
            builder = builder.with_id(SubscriptionId::new(id));
        }

        if let Some(close_on) = close_on {
            builder = builder.close_on(**close_on);
        }

        Ok(builder.await?.to_string())
    }

    /// Unsubscribe
    ///
    /// Returns `true` if the subscription has been unsubscribed.
    pub async fn unsubscribe(&self, id: String) -> Result<bool> {
        Ok(self.inner.unsubscribe(&SubscriptionId::new(id)).await?)
    }

    /// Unsubscribe from all subscriptions
    pub async fn unsubscribe_all(&self) -> Result<()> {
        Ok(self.inner.unsubscribe_all().await?)
    }

    /// Fetch events
    #[uniffi::method(default(timeout = None, policy = None))]
    pub async fn fetch_events(
        &self,
        filter: &Filter,
        timeout: Option<Duration>,
        policy: Option<ReqExitPolicy>,
    ) -> Result<Events> {
        let mut builder = self.inner.fetch_events(filter.deref().clone());

        if let Some(timeout) = timeout {
            builder = builder.timeout(timeout);
        }

        if let Some(policy) = policy {
            builder = builder.policy(policy.into());
        }

        Ok(builder.await?.into())
    }

    /// Count events
    pub async fn count_events(&self, filter: &Filter, timeout: Duration) -> Result<u64> {
        Ok(self
            .inner
            .count_events(filter.deref().clone(), timeout)
            .await? as u64)
    }

    /// Sync events with relays (negentropy reconciliation)
    #[uniffi::method(default(items = None, opts = None))]
    pub async fn sync(
        &self,
        filter: &Filter,
        items: Option<Vec<NegentropyItem>>,
        opts: Option<Arc<SyncOptions>>,
    ) -> Result<RelaySyncSummary> {
        let mut builder = self.inner.sync(filter.deref().clone());

        if let Some(items) = items {
            let items = items.into_iter().map(|item| (**item.id, **item.timestamp));
            builder = builder.items(items);
        }

        if let Some(opts) = opts {
            builder = builder.opts(opts.as_ref().deref().clone());
        }

        Ok(builder.await?.into())
    }
}
