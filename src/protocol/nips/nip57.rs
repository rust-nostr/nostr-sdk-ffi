// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;
use std::sync::Arc;

use nostr::nips::nip57;
use uniffi::Object;

use crate::protocol::event::EventId;
use crate::protocol::key::PublicKey;
use crate::protocol::types::RelayUrl;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct ZapRequestData {
    inner: nip57::ZapRequestData,
}

impl Deref for ZapRequestData {
    type Target = nip57::ZapRequestData;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<nip57::ZapRequestData> for ZapRequestData {
    fn from(inner: nip57::ZapRequestData) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl ZapRequestData {
    #[uniffi::constructor]
    pub fn new(public_key: &PublicKey, relays: Vec<Arc<RelayUrl>>) -> Self {
        Self {
            inner: nip57::ZapRequestData::new(
                **public_key,
                relays.into_iter().map(|u| u.as_ref().deref().clone()),
            ),
        }
    }

    pub fn message(&self, message: &str) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.message(message);
        builder
    }

    pub fn amount(&self, amount: u64) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.amount(amount);
        builder
    }

    pub fn lnurl(&self, lnurl: &str) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.lnurl(lnurl);
        builder
    }

    pub fn event_id(&self, event_id: &EventId) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.event_id(**event_id);
        builder
    }
}
