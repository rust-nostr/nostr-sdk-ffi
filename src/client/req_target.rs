use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

use nostr_sdk::client;
use uniffi::Object;

use crate::protocol::filter::Filter;
use crate::protocol::types::RelayUrl;

/// Request target
#[derive(Object)]
pub struct ReqTarget(client::ReqTarget<'static>);

impl Deref for ReqTarget {
    type Target = client::ReqTarget<'static>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[uniffi::export]
impl ReqTarget {
    /// Automatic relay selection.
    ///
    /// Uses all relays with read permission.
    /// With gossip enabled, also queries relays discovered from public keys in filters.
    #[uniffi::constructor]
    pub fn auto(filters: Vec<Arc<Filter>>) -> Self {
        Self(client::ReqTarget::auto(
            filters.into_iter().map(|f| f.as_ref().deref().clone()),
        ))
    }

    /// Target a specific relay.
    #[uniffi::constructor]
    pub fn single(url: &RelayUrl, filters: Vec<Arc<Filter>>) -> Self {
        Self(client::ReqTarget::single(
            url.deref().clone(),
            filters.into_iter().map(|f| f.as_ref().deref().clone()),
        ))
    }

    /// Target specific relays with their own filters.
    #[uniffi::constructor]
    pub fn manual(targets: HashMap<Arc<RelayUrl>, Vec<Arc<Filter>>>) -> Self {
        let targets = targets.into_iter().map(|(url, filters)| {
            (
                url.as_ref().deref().clone(),
                filters
                    .into_iter()
                    .map(|f| f.as_ref().deref().clone())
                    .collect::<Vec<_>>(),
            )
        });
        Self(client::ReqTarget::manual(targets))
    }
}
