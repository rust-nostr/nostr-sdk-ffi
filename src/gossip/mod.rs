use std::ops::Deref;
use std::sync::Arc;

use nostr_gossip_memory::store::NostrGossipMemory;
#[cfg(feature = "gossip-sqlite")]
use nostr_gossip_sqlite::store::NostrGossipSqlite;
use uniffi::Object;

use crate::error::Result;
use crate::protocol::event::Event;
use crate::protocol::types::RelayUrl;

#[derive(Object)]
pub struct NostrGossip {
    inner: Arc<dyn nostr_gossip::NostrGossip>,
}

impl Deref for NostrGossip {
    type Target = Arc<dyn nostr_gossip::NostrGossip>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl NostrGossip {
    /// Construct a new in-memory gossip store
    #[uniffi::constructor]
    pub fn in_memory() -> Self {
        Self {
            inner: Arc::new(NostrGossipMemory::unbounded()),
        }
    }

    /// Process an event
    ///
    /// Optionally takes the relay URL from where the event comes from.
    #[inline]
    #[uniffi::method(default(relay_url = None))]
    pub async fn process_event(
        &self,
        event: &Event,
        relay_url: Option<Arc<RelayUrl>>,
    ) -> Result<()> {
        let relay_url = relay_url.as_ref().map(|r| r.as_ref().deref());
        self.inner.process(event.deref(), relay_url).await?;
        Ok(())
    }
}

#[cfg(feature = "gossip-sqlite")]
#[uniffi::export]
impl NostrGossip {
    /// Construct a new persistent SQLite gossip store
    #[uniffi::constructor]
    pub async fn sqlite(path: &str) -> Result<Self> {
        Ok(Self {
            inner: Arc::new(NostrGossipSqlite::open(path).await?),
        })
    }
}
