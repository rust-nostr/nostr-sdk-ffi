// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;
use std::sync::Arc;

use nostr::event::{FinalizeEvent, FinalizeEventAsync};
use nostr::nips::{nip44, nip59};
use uniffi::Object;

use crate::error::Result;
use crate::protocol::event::{Event, Tag, UnsignedEvent};
use crate::protocol::key::PublicKey;
use crate::protocol::signer::{
    AsyncNostrSigner, IntermediateAsyncNostrSigner, IntermediateNostrSigner, NostrSigner,
};

/// Seal
///
/// <https://github.com/nostr-protocol/nips/blob/master/59.md>
#[uniffi::constructor]
pub fn nip59_make_seal(
    signer: Arc<dyn NostrSigner>,
    receiver_public_key: &PublicKey,
    rumor: &UnsignedEvent,
) -> Result<Event> {
    let signer = IntermediateNostrSigner::new(signer);
    let event = nip59::GiftWrapSealBuilder::new(rumor.deref().clone(), **receiver_public_key)
        .finalize(&signer)?;
    Ok(event.into())
}

/// Seal
///
/// <https://github.com/nostr-protocol/nips/blob/master/59.md>
#[uniffi::constructor]
pub async fn nip59_make_seal_async(
    signer: Arc<dyn AsyncNostrSigner>,
    receiver_public_key: &PublicKey,
    rumor: &UnsignedEvent,
) -> Result<Event> {
    let signer = IntermediateAsyncNostrSigner::new(signer);
    let event = nip59::GiftWrapSealBuilder::new(rumor.deref().clone(), **receiver_public_key)
        .finalize_async(&signer)
        .await?;
    Ok(event.into())
}

/// Build Gift Wrap from Seal
///
/// <https://github.com/nostr-protocol/nips/blob/master/59.md>
#[uniffi::export(default(extra_tags = []))]
pub fn nip59_make_gift_wrap_from_seal(
    receiver: &PublicKey,
    seal: &Event,
    extra_tags: Vec<Arc<Tag>>,
) -> Result<Event> {
    let keys = nostr::Keys::generate();
    let content = nip44::encrypt(
        keys.secret_key(),
        receiver.deref(),
        seal.deref().as_json(),
        nip44::Version::default(),
    )?;
    let mut tags: Vec<nostr::Tag> = extra_tags
        .into_iter()
        .map(|t| t.as_ref().deref().clone())
        .collect();
    tags.push(nostr::Tag::public_key(**receiver));
    let event = nostr::EventBuilder::new(nostr::Kind::GiftWrap, content)
        .tags(tags)
        .custom_created_at(nostr::Timestamp::tweaked(0..172800))
        .finalize(&keys)?;
    Ok(event.into())
}

/// Build Gift Wrap
///
/// <https://github.com/nostr-protocol/nips/blob/master/59.md>
#[uniffi::export(default(extra_tags = []))]
pub fn nip59_make_gift_wrap(
    signer: Arc<dyn NostrSigner>,
    receiver_pubkey: &PublicKey,
    rumor: &UnsignedEvent,
    extra_tags: Vec<Arc<Tag>>,
) -> Result<Event> {
    let signer = IntermediateNostrSigner::new(signer);
    let event = nip59::GiftWrapBuilder::new(**receiver_pubkey, rumor.deref().clone())
        .extra_tags(extra_tags.into_iter().map(|t| t.as_ref().deref().clone()))
        .finalize(&signer)?;
    Ok(event.into())
}

/// Build Gift Wrap
///
/// <https://github.com/nostr-protocol/nips/blob/master/59.md>
#[uniffi::export(async_runtime = "tokio", default(extra_tags = []))]
pub async fn nip59_make_gift_wrap_async(
    signer: Arc<dyn AsyncNostrSigner>,
    receiver_pubkey: &PublicKey,
    rumor: &UnsignedEvent,
    extra_tags: Vec<Arc<Tag>>,
) -> Result<Event> {
    let signer = IntermediateAsyncNostrSigner::new(signer);
    let event = nip59::GiftWrapBuilder::new(**receiver_pubkey, rumor.deref().clone())
        .extra_tags(extra_tags.into_iter().map(|t| t.as_ref().deref().clone()))
        .finalize_async(&signer)
        .await?;
    Ok(event.into())
}

/// Unwrapped Gift Wrap
///
/// <https://github.com/nostr-protocol/nips/blob/master/59.md>
#[derive(Debug, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct UnwrappedGift {
    inner: nip59::UnwrappedGift,
}

impl From<nip59::UnwrappedGift> for UnwrappedGift {
    fn from(inner: nip59::UnwrappedGift) -> Self {
        Self { inner }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl UnwrappedGift {
    // `#[uniffi::export(async_runtime = "tokio")]` require an async method
    async fn _none(&self) {}

    /// Unwrap Gift Wrap event
    ///
    /// Internally verify the `seal` event
    #[uniffi::constructor]
    pub fn from_gift_wrap(signer: Arc<dyn NostrSigner>, gift_wrap: &Event) -> Result<Self> {
        let signer = IntermediateNostrSigner::new(signer);
        Ok(Self {
            inner: nip59::UnwrappedGift::from_gift_wrap(&signer, gift_wrap.deref())?,
        })
    }

    /// Unwrap Gift Wrap event
    ///
    /// Internally verify the `seal` event
    #[uniffi::constructor]
    pub async fn from_gift_wrap_async(
        signer: Arc<dyn AsyncNostrSigner>,
        gift_wrap: &Event,
    ) -> Result<Self> {
        let signer = IntermediateAsyncNostrSigner::new(signer);
        Ok(Self {
            inner: nip59::UnwrappedGift::from_gift_wrap_async(&signer, gift_wrap.deref()).await?,
        })
    }

    /// Get sender public key
    pub fn sender(&self) -> PublicKey {
        self.inner.sender.into()
    }

    /// Get rumor
    pub fn rumor(&self) -> UnsignedEvent {
        self.inner.rumor.clone().into()
    }
}
