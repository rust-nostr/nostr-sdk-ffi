use std::ops::Deref;
use std::sync::Arc;

use nostr_gossip_memory::store::NostrGossipMemory;
#[cfg(feature = "gossip-sqlite")]
use nostr_gossip_sqlite::store::NostrGossipSqlite;
use uniffi::Object;

use crate::error::Result;

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

#[uniffi::export]
impl NostrGossip {
    /// Construct a new in-memory gossip store
    #[uniffi::constructor]
    pub fn in_memory() -> Self {
        Self {
            inner: Arc::new(NostrGossipMemory::unbounded()),
        }
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
