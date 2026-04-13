// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use nostr::SubscriptionId;
use nostr_sdk::client;
use uniffi::Object;

mod builder;
mod notification;
mod output;
mod req_target;
mod stream;

use self::output::{ClientSyncSummaryOutput, Output, SendEventOutput, SubscribeOutput};
use self::req_target::ReqTarget;
use self::stream::{ClientEventStream, ClientNotificationStream};
use crate::database::NostrDatabase;
use crate::database::events::Events;
use crate::error::Result;
use crate::monitor::Monitor;
use crate::protocol::event::Event;
use crate::protocol::filter::Filter;
use crate::protocol::signer::NostrSigner;
use crate::protocol::types::RelayUrl;
use crate::relay::capabilities::RelayCapabilities;
use crate::relay::options::{SubscribeAutoCloseOptions, SyncOptions};
use crate::relay::{Relay, RelayOptions, ReqExitPolicy};

#[derive(Object)]
pub struct Client {
    inner: client::Client,
}

impl From<client::Client> for Client {
    fn from(inner: client::Client) -> Self {
        Self { inner }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl Client {
    /// Construct a new default client
    ///
    /// Use the ClientBuilder to configure the client (i.e., set a signer).
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            inner: client::Client::default(),
        }
    }

    /// Auto authenticate to relays (default: true)
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/42.md>
    pub fn automatic_authentication(&self, enable: bool) {
        self.inner.automatic_authentication(enable);
    }

    /// Get current nostr signer
    ///
    /// Returns None if no signer is configured.
    pub fn signer(&self) -> Option<Arc<NostrSigner>> {
        let signer = self.inner.signer()?;
        Some(Arc::new(signer.clone().into()))
    }

    pub fn database(&self) -> NostrDatabase {
        self.inner.database().clone().into()
    }

    /// Get monitor
    pub fn monitor(&self) -> Option<Arc<Monitor>> {
        self.inner.monitor().cloned().map(|m| Arc::new(m.into()))
    }

    pub async fn shutdown(&self) {
        self.inner.shutdown().await
    }

    /// Get a new notification stream
    ///
    /// The stream terminates when the client shutdowns.
    ///
    /// <div class="warning">When you call this method, you subscribe to the notifications channel from that precise moment. Anything received by relay/s before that moment is not included in the channel!</div>
    #[inline]
    pub fn notifications(&self) -> ClientNotificationStream {
        self.inner.notifications().into()
    }

    /// Get relays with `READ` or `WRITE` flags
    pub async fn relays(&self) -> HashMap<Arc<RelayUrl>, Arc<Relay>> {
        self.inner
            .relays()
            .await
            .into_iter()
            .map(|(u, r)| (Arc::new(u.into()), Arc::new(r.into())))
            .collect()
    }

    /// Get a previously added Relay by URL.
    ///
    /// It returns the relay only if it has already been added to the client via Client::add_relay.
    ///
    /// Returns null if the relay has not been found in the pool.
    pub async fn relay(&self, url: &RelayUrl) -> Result<Option<Arc<Relay>>> {
        let relay = self.inner.relay(url.deref()).await?;
        Ok(relay.map(|r| Arc::new(r.into())))
    }

    /// Add relay
    ///
    /// By default, relays added with this method will have both `READ` and `WRITE` capabilities enabled.
    /// If the relay already exists, the capabilities will be updated and `false` returned.
    ///
    /// To add a relay with specific capabilities, use the `capabilities` argument.
    ///
    /// Connection is **NOT** automatically started with relay!
    #[uniffi::method(default(opts = None))]
    pub async fn add_relay(
        &self,
        url: &RelayUrl,
        capabilities: Option<Arc<RelayCapabilities>>,
        opts: Option<Arc<RelayOptions>>,
    ) -> Result<bool> {
        let mut builder = self.inner.add_relay(url.deref());

        if let Some(capabilities) = capabilities {
            builder = builder.capabilities(**capabilities);
        }

        if let Some(opts) = opts {
            builder = builder.opts(opts.as_ref().deref().clone());
        }

        Ok(builder.await?)
    }

    /// Remove and disconnect relay
    ///
    /// If the relay has `GOSSIP`, it will not be removed from the pool and its
    /// capabilities will be updated (remove `READ`, `WRITE` and `DISCOVERY` capabilities).
    ///
    /// If the `force` argument is `true`, the relay will be removed even if it's in use for the gossip model or other service!
    #[uniffi::method(default(force = false))]
    pub async fn remove_relay(&self, url: &RelayUrl, force: bool) -> Result<()> {
        let mut builder = self.inner.remove_relay(url.deref());

        if force {
            builder = builder.force();
        }

        Ok(builder.await?)
    }

    /// Disconnect and remove all relays
    ///
    /// Some relays (i.e., the gossip ones) will not be disconnected and removed unless you
    /// set `force` to `true`.
    #[uniffi::method(default(force = false))]
    pub async fn remove_all_relays(&self, force: bool) -> Result<()> {
        let mut builder = self.inner.remove_all_relays();

        if force {
            builder = builder.force();
        }

        Ok(builder.await?)
    }

    /// Connect to a previously added relay
    pub async fn connect_relay(&self, url: &RelayUrl) -> Result<()> {
        Ok(self.inner.connect_relay(url.deref()).await?)
    }

    pub async fn disconnect_relay(&self, url: &RelayUrl) -> Result<()> {
        Ok(self.inner.disconnect_relay(url.deref()).await?)
    }

    /// Connect to relays
    ///
    /// Attempts to initiate a connection with relays.
    ///
    /// At most **one connection per relay** is allowed at any time.
    /// If a relay is already connected or currently attempting to connect,
    /// this method does nothing for that relay.
    ///
    /// If a relay is disconnected, sleeping, or otherwise inactive, a
    /// background task is spawned to initiate a connection.
    ///
    /// For further details, see the documentation of [`Relay::connect`].
    ///
    /// # Configuration
    ///
    /// By default:
    ///
    /// - Doesn't wait that relays connect
    ///
    /// To customize this behavior, the arguments can be adjusted:
    ///
    /// - `and_wait`: wait for relays connections at most for the specified `timeout`
    #[uniffi::method(default(and_wait = None))]
    pub async fn connect(&self, and_wait: Option<Duration>) {
        let mut builder = self.inner.connect();

        if let Some(timeout) = and_wait {
            builder = builder.and_wait(timeout);
        }

        builder.await
    }

    /// Try to establish a connection with relays.
    ///
    /// # Overview
    ///
    /// Attempts to initiate a connection with relays.
    ///
    /// At most **one connection per relay** is allowed at any time.
    /// If a relay is already connected or currently attempting to connect,
    /// this method does nothing for that relay.
    ///
    /// If the initial connection attempt succeeds, a background task is spawned
    /// to maintain the connection and handle future reconnections.
    /// If the initial attempt fails, no background task is spawned and no
    /// automatic retries are scheduled.
    ///
    /// Use [`Client::connect`] if you want to always spawn a background
    /// connection task, regardless of whether the initial attempt succeeds.
    ///
    /// For further details, see the documentation of [`Relay::try_connect`].
    ///
    /// # Configuration
    ///
    /// By default:
    ///
    /// - Connection timeout is set to 60 secs
    ///
    /// To customize this behavior, the arguments can be adjusted:
    ///
    /// - `timeout`: set a maximum timeout
    pub async fn try_connect(&self, timeout: Option<Duration>) -> Output {
        let mut builder = self.inner.try_connect();

        if let Some(timeout) = timeout {
            builder = builder.timeout(timeout);
        }

        builder.await.into()
    }

    /// Disconnect from all relays
    pub async fn disconnect(&self) {
        self.inner.disconnect().await
    }

    pub async fn subscriptions(&self) -> HashMap<String, HashMap<Arc<RelayUrl>, Vec<Arc<Filter>>>> {
        self.inner
            .subscriptions()
            .await
            .into_iter()
            .map(|(id, f)| {
                let map = f
                    .into_iter()
                    .map(|(url, filters)| {
                        (
                            Arc::new(url.into()),
                            filters.into_iter().map(|f| Arc::new(f.into())).collect(),
                        )
                    })
                    .collect();
                (id.to_string(), map)
            })
            .collect()
    }

    pub async fn subscription(&self, id: String) -> HashMap<Arc<RelayUrl>, Vec<Arc<Filter>>> {
        self.inner
            .subscription(&SubscriptionId::new(id))
            .await
            .into_iter()
            .map(|(url, filters)| {
                (
                    Arc::new(url.into()),
                    filters.into_iter().map(|f| Arc::new(f.into())).collect(),
                )
            })
            .collect()
    }

    /// Subscribe to events from relays.
    ///
    /// # Overview
    ///
    /// Creates a long-lived event subscription.
    ///
    /// The subscription remains active until it is explicitly closed or until
    /// auto-close conditions are met.
    ///
    /// For short-lived, request-style event streams, use [`Client::stream_events`] or [`Client::fetch_events`].
    ///
    /// # Configuration
    ///
    /// By default:
    ///
    /// - a random subscription ID is generated
    /// - no auto-close condition are set
    ///
    /// The args can be configured before execution:
    ///
    /// - `id`: set an explicit subscription ID
    /// - `close_on`: configure automatic closing conditions
    ///
    /// # Target Resolution
    ///
    /// The request target determines which relays are queried:
    ///
    /// - [`ReqTarget::auto`]: Sends the subscription to all relays with
    ///   `READ` capability. If gossip is enabled
    ///   ([`ClientBuilder::gossip`]), NIP-65 relays are also included.
    /// - [`ReqTarget::single`] / [`ReqTarget::manual`]: Sends the subscription only to
    ///   the explicitly specified relays.
    ///
    /// # Event Semantics
    ///
    /// - Event signatures are **validated**.
    /// - Events are **verified against the requested filters** if
    ///   [`ClientBuilder::verify_subscriptions`] is enabled.
    /// - Event replacements, deletions, and other stateful event semantics
    ///   depend on the database implementation in use.
    ///
    /// # Lifetime
    ///
    /// The subscription terminates when:
    ///
    /// - It is explicitly closed,
    /// - Auto-close conditions are met (if configured),
    /// - Or the relay closes it remotely.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The resolved target contains no relays,
    /// - A specified relay does not exist in the pool,
    /// - Target resolution fails.
    #[uniffi::method(default(id = None, close_on = None))]
    pub async fn subscribe(
        &self,
        target: &ReqTarget,
        id: Option<String>,
        close_on: Option<Arc<SubscribeAutoCloseOptions>>,
    ) -> Result<SubscribeOutput> {
        let mut builder = self.inner.subscribe(target.deref().clone());

        if let Some(id) = id {
            builder = builder.with_id(SubscriptionId::new(id));
        }

        if let Some(close_on) = close_on {
            builder = builder.close_on(**close_on);
        }

        Ok(builder.await?.into())
    }

    pub async fn unsubscribe(&self, subscription_id: String) -> Result<Output> {
        Ok(self
            .inner
            .unsubscribe(&SubscriptionId::new(subscription_id))
            .await?
            .into())
    }

    pub async fn unsubscribe_all(&self) -> Result<Output> {
        Ok(self.inner.unsubscribe_all().await?.into())
    }

    /// Synchronize events with relays using negentropy.
    ///
    /// # Overview
    ///
    /// Performs a negentropy-based reconciliation between the local database
    /// and one or more relays.
    ///
    /// # Configuration
    ///
    /// The args can be configured before execution:
    ///
    /// - `:with`: explicitly select which relays to synchronize with
    /// - `opts`: configure reconciliation behavior
    ///
    /// If no relays are explicitly specified, the target set is resolved
    /// automatically (see *Target Resolution*).
    ///
    /// # Target Resolution
    ///
    /// The set of relays to synchronize with is determined as follows:
    ///
    /// - If relays are explicitly provided via `with` arg, only those
    ///   relays are used.
    /// - Otherwise, if gossip is enabled ([`ClientBuilder::gossip`]), NIP-65 relays
    ///   are automatically discovered and used as targets.
    /// - Otherwise, all relays in the pool with `READ` or `WRITE` capabilities are used.
    ///
    /// Each target relay receives the same filter, scoped to the events relevant
    /// for reconciliation.
    ///
    /// # Reconciliation Semantics
    ///
    /// - Reconciliation is performed using NIP-77 negentropy
    ///   (<https://github.com/nostr-protocol/nips/blob/master/77.md>).
    /// - Event transfer occurs **only** for events determined to be missing
    ///   on either side.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - Target resolution fails,
    /// - A specified relay URL is invalid,
    /// - Database access fails,
    /// - Or reconciliation cannot be initiated.
    ///
    /// Relay-specific failures during event transfer are reported in the
    /// returned [`ClientSyncSummary`].
    #[uniffi::method(default(with = None, opts = None))]
    pub async fn sync(
        &self,
        filter: &Filter,
        with: Option<Vec<Arc<RelayUrl>>>,
        opts: Option<Arc<SyncOptions>>,
    ) -> Result<ClientSyncSummaryOutput> {
        let mut builder = self.inner.sync(filter.deref().clone());

        if let Some(with) = with {
            builder = builder.with(with.into_iter().map(|u| u.as_ref().deref().clone()));
        }

        if let Some(opts) = opts {
            builder = builder.opts(opts.as_ref().deref().clone());
        }

        Ok(builder.await?.into())
    }

    /// Fetch events from relays.
    ///
    /// # Overview
    ///
    /// Creates a short-lived event subscription and returns a list of events.
    /// Compared to [`Client::stream_events`], this buffers events internally and returns them only after the stream terminates.
    ///
    /// For long-lived subscriptions, use [`Client::subscribe`].
    ///
    /// # Configuration
    ///
    /// By default:
    ///
    /// - No timeout is set
    /// - Exit policy is [`ReqExitPolicy::ExitOnEOSE`]
    ///
    /// To customize this behavior, the args can be
    /// configured before awaiting it:
    ///
    /// - `timeout`: set a maximum duration for the stream
    /// - `policy`: control when the stream terminates
    ///
    /// # Target Resolution, Event Semantics and Termination
    ///
    /// See [`Client::stream_events`] for details on:
    ///
    /// - Target resolution
    /// - Event semantics
    /// - Stream termination conditions
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The resolved target contains no relays,
    /// - A specified relay does not exist in the pool,
    /// - Target resolution fails.
    #[uniffi::method(default(timeout = None, policy = None))]
    pub async fn fetch_events(
        &self,
        target: &ReqTarget,
        timeout: Option<Duration>,
        policy: Option<ReqExitPolicy>,
    ) -> Result<Events> {
        let mut builder = self.inner.fetch_events(target.deref().clone());

        if let Some(timeout) = timeout {
            builder = builder.timeout(timeout);
        }

        if let Some(policy) = policy {
            builder = builder.policy(policy.into());
        }

        Ok(builder.await?.into())
    }

    /// Stream events from relays.
    ///
    /// # Overview
    ///
    /// Creates a short-lived event subscription and returns a stream of events.
    ///
    /// For long-lived subscriptions, use [`Client::subscribe`].
    ///
    /// # Configuration
    ///
    /// By default:
    ///
    /// - No timeout is set
    /// - Exit policy is [`ReqExitPolicy::ExitOnEOSE`](crate::relay::ReqExitPolicy::ExitOnEOSE)
    ///
    /// To customize this behavior, the args can be
    /// configured before awaiting it:
    ///
    /// - `id`: use a specific subscription ID
    /// - `timeout`: set a maximum duration for the stream
    /// - `policy`: control when the stream terminates
    ///
    /// # Target Resolution
    ///
    /// The request target determines which relays are queried:
    ///
    /// - [`ReqTarget::auto`]: Streams events from all relays with
    ///   `READ`. If gossip is enabled
    ///   ([`ClientBuilder::gossip`]), NIP-65 relays are also included.
    /// - [`ReqTarget::single`] / [`ReqTarget::manual`]: Streams events only from
    ///   the explicitly specified relays.
    ///
    /// # Event Semantics
    ///
    /// - Events are **deduplicated** across relays by event ID.
    /// - Event signatures are **validated**.
    /// - Events are **verified against the requested filters** if
    ///   [`ClientBuilder::verify_subscriptions`] is enabled.
    /// - Event replacements, deletions, and other stateful event semantics
    ///   depend on the database implementation in use.
    ///
    /// # Termination
    ///
    /// The stream terminates when:
    ///
    /// - The exit policy condition is met (i.e., EOSE),
    /// - All relay streams terminate,
    /// - Or an optional timeout expires.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The resolved target contains no relays,
    /// - A specified relay does not exist in the pool,
    /// - Target resolution fails.
    ///
    /// Network or relay-specific errors are reported **inside the stream**
    /// as `Err(relay::Error)` items.
    pub async fn stream_events(
        &self,
        target: &ReqTarget,
        id: Option<String>,
        timeout: Option<Duration>,
        policy: Option<ReqExitPolicy>,
    ) -> Result<ClientEventStream> {
        let mut builder = self.inner.stream_events(target.deref().clone());

        if let Some(id) = id {
            builder = builder.with_id(SubscriptionId::new(id));
        }

        if let Some(timeout) = timeout {
            builder = builder.timeout(timeout);
        }

        if let Some(policy) = policy {
            builder = builder.policy(policy.into());
        }

        Ok(builder.await?.into())
    }

    /// Send event
    ///
    /// Send event to all relays with `WRITE` flag.
    /// If `gossip` is enabled (see `Options`) the event will be sent also to NIP65 relays (automatically discovered).
    pub async fn send_event(&self, event: &Event) -> Result<SendEventOutput> {
        Ok(self.inner.send_event(event.deref()).await?.into())
    }

    // /// Signs the `EventBuilder` into an `Event` using the `NostrSigner`
    // pub async fn sign_event_builder(&self, builder: &EventBuilder) -> Result<Event> {
    //     Ok(self
    //         .inner
    //         .sign_event_builder(builder.deref().clone())
    //         .await?
    //         .into())
    // }
    //
    // /// Take an `EventBuilder`, sign it by using the `NostrSigner` and broadcast to relays (check `send_event` method for more details)
    // ///
    // /// Rise an error if the `NostrSigner` is not set.
    // pub async fn send_event_builder(&self, builder: &EventBuilder) -> Result<SendEventOutput> {
    //     Ok(self
    //         .inner
    //         .send_event_builder(builder.deref().clone())
    //         .await?
    //         .into())
    // }

    // /// Take an `EventBuilder`, sign it by using the `NostrSigner` and broadcast to specific relays.
    // ///
    // /// Rise an error if the `NostrSigner` is not set.
    // pub async fn send_event_builder_to(
    //     &self,
    //     urls: Vec<Arc<RelayUrl>>,
    //     builder: &EventBuilder,
    // ) -> Result<SendEventOutput> {
    //     let urls = urls.into_iter().map(|u| u.as_ref().deref().clone());
    //     Ok(self
    //         .inner
    //         .send_event_builder_to(urls, builder.deref().clone())
    //         .await?
    //         .into())
    // }
}
