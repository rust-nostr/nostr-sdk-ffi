use std::fmt;
use std::net::{IpAddr, SocketAddr};
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

use nostr::prelude::BoxedFuture;
use nostr_relay_builder::{builder, local};
use uniffi::{Enum, Object, Record};

use crate::database::NostrDatabase;
use crate::error::Result;
use crate::protocol::event::Event;
use crate::protocol::filter::Filter;
use crate::protocol::types::RelayUrl;

/// Rate limit
#[derive(Record)]
pub struct RateLimit {
    /// Max active REQs
    pub max_reqs: u64,
    /// Max events per minutes
    pub notes_per_minute: u32,
}

impl From<RateLimit> for builder::RateLimit {
    fn from(rate_limit: RateLimit) -> Self {
        Self {
            max_reqs: rate_limit.max_reqs as usize,
            notes_per_minute: rate_limit.notes_per_minute,
        }
    }
}

/// Generic plugin policy response
#[derive(Enum)]
pub enum PolicyResult {
    /// Policy enforces that the event/query should be accepted
    Accept,
    /// Policy enforces that the event/query should be rejected
    Reject {
        /// Rejection reason
        reason: String,
    },
}

impl From<PolicyResult> for builder::PolicyResult {
    fn from(policy_result: PolicyResult) -> Self {
        match policy_result {
            PolicyResult::Accept => Self::Accept,
            PolicyResult::Reject { reason } => Self::Reject(reason),
        }
    }
}

/// Custom policy for accepting events into the relay database
#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait WritePolicy: Send + Sync {
    /// Check if the policy should accept an event
    async fn admit_event(&self, event: Arc<Event>, socket_addr: String) -> PolicyResult;
}

struct WritePolicyAdapter(Arc<dyn WritePolicy>);

impl fmt::Debug for WritePolicyAdapter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WritePolicyAdapter").finish()
    }
}

impl builder::WritePolicy for WritePolicyAdapter {
    fn admit_event<'a>(
        &'a self,
        event: &'a nostr::Event,
        addr: &'a SocketAddr,
    ) -> BoxedFuture<'a, builder::PolicyResult> {
        Box::pin(async move {
            self.0
                .admit_event(Arc::new(event.clone().into()), addr.to_string())
                .await
                .into()
        })
    }
}

/// Filters REQ's to the internal relay database
#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait QueryPolicy: Send + Sync {
    /// Check if the policy should accept a query
    async fn admit_query(&self, query: Arc<Filter>, socket_addr: String) -> PolicyResult;
}

struct QueryPolicyAdapter(Arc<dyn QueryPolicy>);

impl fmt::Debug for QueryPolicyAdapter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("QueryPolicyAdapter").finish()
    }
}

impl builder::QueryPolicy for QueryPolicyAdapter {
    fn admit_query<'a>(
        &'a self,
        query: &'a nostr::Filter,
        addr: &'a SocketAddr,
    ) -> BoxedFuture<'a, builder::PolicyResult> {
        Box::pin(async move {
            self.0
                .admit_query(Arc::new(query.clone().into()), addr.to_string())
                .await
                .into()
        })
    }
}

/// NIP42 mode
#[derive(Enum)]
pub enum RelayBuilderNip42Mode {
    /// Require authentication for writing
    Write,
    /// Require authentication for reading
    Read,
    /// Always require authentication
    Both,
}

impl From<RelayBuilderNip42Mode> for builder::RelayBuilderNip42Mode {
    fn from(mode: RelayBuilderNip42Mode) -> Self {
        match mode {
            RelayBuilderNip42Mode::Write => Self::Write,
            RelayBuilderNip42Mode::Read => Self::Read,
            RelayBuilderNip42Mode::Both => Self::Both,
        }
    }
}

/// NIP42 options
#[derive(Record)]
pub struct RelayBuilderNip42 {
    /// Mode
    pub mode: RelayBuilderNip42Mode,
}

impl From<RelayBuilderNip42> for builder::RelayBuilderNip42 {
    fn from(value: RelayBuilderNip42) -> Self {
        Self {
            mode: value.mode.into(),
        }
    }
}

/// Relay builder
#[derive(Clone, Default, Object)]
pub struct RelayBuilder {
    inner: builder::RelayBuilder,
}

impl Deref for RelayBuilder {
    type Target = builder::RelayBuilder;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[uniffi::export]
impl RelayBuilder {
    /// Construct new default relay builder
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set IP address
    pub fn addr(&self, ip_addr: &str) -> Result<Self> {
        let ip = IpAddr::from_str(ip_addr)?;
        let mut builder = self.clone();
        builder.inner = builder.inner.addr(ip);
        Ok(builder)
    }

    /// Set port
    pub fn port(&self, port: u16) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.port(port);
        builder
    }

    /// Set database
    pub fn database(&self, database: &NostrDatabase) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.database(database.deref().clone());
        builder
    }

    /// Set rate limit
    pub fn rate_limit(&self, limit: RateLimit) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.rate_limit(limit.into());
        builder
    }

    /// Require NIP42 authentication
    pub fn nip42(&self, opts: RelayBuilderNip42) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.nip42(opts.into());
        builder
    }

    /// Set number of max connections allowed
    pub fn max_connections(&self, max: u64) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.max_connections(max as usize);
        builder
    }

    /// Sets the maximum subscription ID length. Defaults 250.
    pub fn max_subid_length(&self, max: u64) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.max_subid_length(max as usize);
        builder
    }

    /// Sets the maximum limit for the filter. If the filter's limit exceeds
    /// this value, it will fallback to this number.
    pub fn max_filter_limit(&self, max: u64) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.max_filter_limit(max as usize);
        builder
    }

    /// Sets the default filter limit when no limit is specified. Defaults 500.
    pub fn default_filter_limit(&self, limit: u64) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.default_filter_limit(limit as usize);
        builder
    }

    /// If enabled, NIP-42 will be used for DMs, returning GiftWrap events for
    /// the mentioned public key only.
    pub fn auth_dm(&self, enable: bool) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.auth_dm(enable);
        builder
    }

    /// Sets the minimum Proof of Work difficulty.
    ///
    /// Only values `> 0` are accepted!
    pub fn min_pow(&self, difficulty: u8) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.min_pow(difficulty);
        builder
    }

    /// Add a write policy plugin
    pub fn write_policy(&self, policy: Arc<dyn WritePolicy>) -> Self {
        let mut builder = self.clone();
        let adapter = WritePolicyAdapter(policy);
        builder.inner = builder.inner.write_policy(adapter);
        builder
    }

    /// Add a query policy plugin
    pub fn query_policy(&self, policy: Arc<dyn QueryPolicy>) -> Self {
        let mut builder = self.clone();
        let adapter = QueryPolicyAdapter(policy);
        builder.inner = builder.inner.query_policy(adapter);
        builder
    }
}

/// A local nostr relay
///
/// This is automatically shutdown when all instances/clones are dropped!
#[derive(Object)]
pub struct LocalRelay {
    inner: local::LocalRelay,
}

#[uniffi::export(async_runtime = "tokio")]
impl LocalRelay {
    /// Construct a new relay
    #[uniffi::constructor]
    pub fn new(builder: &RelayBuilder) -> Self {
        Self {
            inner: local::LocalRelay::new(builder.deref().clone()),
        }
    }

    /// Run the local relay
    pub async fn run(&self) -> Result<()> {
        self.inner.run().await?;
        Ok(())
    }

    /// Get url
    #[inline]
    pub async fn url(&self) -> RelayUrl {
        self.inner.url().await.into()
    }

    /// Send event to subscribers
    ///
    /// Return `true` if the event is successfully sent.
    ///
    /// This method doesn't save the event into the database!
    /// It's intended to be used ONLY when the database is shared with other apps (i.e. with the nostr-sdk `Client`).
    pub fn notify_event(&self, event: &Event) -> bool {
        self.inner.notify_event(event.deref().clone())
    }

    /// Shutdown relay
    #[inline]
    pub fn shutdown(&self) {
        self.inner.shutdown();
    }
}
