// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::num::NonZeroU8;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

use nostr::JsonUtil;
use nostr::secp256k1::schnorr::Signature;
use uniffi::Object;

use super::EventId;
use crate::error::{NostrSdkError, Result};
use crate::protocol::event::{Event, Kind, Tags, Timestamp};
use crate::protocol::key::{Keys, PublicKey};
use crate::protocol::nips::nip13::{
    AsyncPowAdapter, IntermediateAsyncPowAdapter, IntermediatePowAdapter, PowAdapter,
};
use crate::protocol::signer::{
    AsyncNostrSigner, IntermediateAsyncNostrSigner, IntermediateNostrSigner, NostrSigner,
};

#[derive(Debug, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct UnsignedEvent {
    inner: nostr::UnsignedEvent,
}

impl Deref for UnsignedEvent {
    type Target = nostr::UnsignedEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<nostr::UnsignedEvent> for UnsignedEvent {
    fn from(inner: nostr::UnsignedEvent) -> Self {
        Self { inner }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl UnsignedEvent {
    pub fn id(&self) -> Option<Arc<EventId>> {
        self.inner.id.map(|id| Arc::new(id.into()))
    }

    pub fn author(&self) -> PublicKey {
        self.inner.pubkey.into()
    }

    pub fn created_at(&self) -> Timestamp {
        self.inner.created_at.into()
    }

    pub fn kind(&self) -> Kind {
        self.inner.kind.into()
    }

    pub fn tags(&self) -> Tags {
        self.inner.tags.clone().into()
    }

    pub fn content(&self) -> String {
        self.inner.content.clone()
    }

    /// Mine an unsigned event synchronously
    pub fn mine(&self, adapter: Arc<dyn PowAdapter>, difficulty: u8) -> Result<Self> {
        let inner: nostr::UnsignedEvent = self.inner.clone();
        let adapter = IntermediatePowAdapter::new(adapter);
        let difficulty = NonZeroU8::new(difficulty).ok_or(NostrSdkError::NonZeroDifficulty)?;
        let unsigned: nostr::UnsignedEvent = inner.mine(&adapter, difficulty)?;
        Ok(unsigned.into())
    }

    /// Mine an unsigned event asynchronously
    pub async fn mine_async(
        &self,
        adapter: Arc<dyn AsyncPowAdapter>,
        difficulty: u8,
    ) -> Result<Self> {
        let inner: nostr::UnsignedEvent = self.inner.clone();
        let adapter = IntermediateAsyncPowAdapter::new(adapter);
        let difficulty = NonZeroU8::new(difficulty).ok_or(NostrSdkError::NonZeroDifficulty)?;
        let unsigned: nostr::UnsignedEvent = inner.mine_async(&adapter, difficulty).await?;
        Ok(unsigned.into())
    }

    /// Sign an unsigned event
    pub fn sign(&self, signer: Arc<dyn NostrSigner>) -> Result<Event> {
        let signer = IntermediateNostrSigner::new(signer);
        let event = self.inner.clone().sign(&signer)?;
        Ok(event.into())
    }

    /// Sign an unsigned event
    pub async fn sign_async(&self, signer: Arc<dyn AsyncNostrSigner>) -> Result<Event> {
        let signer = IntermediateAsyncNostrSigner::new(signer);
        let event = self.inner.clone().sign_async(&signer).await?;
        Ok(event.into())
    }

    /// Sign an unsigned event with keys signer
    ///
    /// Internally: calculate event ID (if not set), sign it, compose and verify event.
    pub fn sign_with_keys(&self, keys: &Keys) -> Result<Event> {
        Ok(Event::from(
            self.inner.clone().sign_with_keys(keys.deref())?,
        ))
    }

    /// Add signature to unsigned event
    ///
    /// Internally verify the event.
    pub fn add_signature(&self, sig: &str) -> Result<Event> {
        let sig = Signature::from_str(sig)?;
        Ok(Event::from(self.inner.clone().add_signature(sig)?))
    }

    #[uniffi::constructor]
    pub fn from_json(json: String) -> Result<Self> {
        Ok(Self {
            inner: nostr::UnsignedEvent::from_json(json)?,
        })
    }

    pub fn as_json(&self) -> Result<String> {
        Ok(self.inner.try_as_json()?)
    }

    pub fn as_pretty_json(&self) -> Result<String> {
        Ok(self.inner.try_as_pretty_json()?)
    }
}
