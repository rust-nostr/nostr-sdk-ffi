use std::ops::Deref;
use std::sync::Arc;

use nostr_gossip_memory::store::NostrGossipMemory;
use uniffi::Object;

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
