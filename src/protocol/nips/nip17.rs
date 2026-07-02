// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use nostr::event::{FinalizeEvent, FinalizeEventAsync};
use nostr::nips::nip17;

use crate::error::Result;
use crate::protocol::event::{Event, Tag};
use crate::protocol::key::PublicKey;
use crate::protocol::signer::{
    AsyncNostrSigner, IntermediateAsyncNostrSigner, IntermediateNostrSigner, NostrSigner,
};
use crate::protocol::types::RelayUrl;

fn make_builder(
    receiver: &PublicKey,
    message: &str,
    expiration: Option<Duration>,
    rumor_extra_tags: Vec<Arc<Tag>>,
) -> nip17::PrivateDirectMessageBuilder {
    let mut builder = nip17::PrivateDirectMessageBuilder::new(**receiver, message)
        .rumor_extra_tags(
            rumor_extra_tags
                .into_iter()
                .map(|t| t.as_ref().deref().clone()),
        );

    if let Some(duration) = expiration {
        builder = builder.expiration(duration);
    }

    builder
}

/// Private Direct message
///
/// The expiration tag is relative to the gift wrap's `created_at`
/// so it doesn't leak the real send time.
/// Choose a `duration` greater than the 2 days
/// or the event may be created in an expired state.
///
/// <https://github.com/nostr-protocol/nips/blob/master/17.md>
#[uniffi::export(default(expiration = None, rumor_extra_tags = []))]
pub fn nip17_make_private_msg(
    signer: Arc<dyn NostrSigner>,
    receiver: &PublicKey,
    message: &str,
    expiration: Option<Duration>,
    rumor_extra_tags: Vec<Arc<Tag>>,
) -> Result<Event> {
    let signer = IntermediateNostrSigner::new(signer);
    let builder = make_builder(receiver, message, expiration, rumor_extra_tags);
    let event = builder.finalize(&signer)?;
    Ok(event.into())
}

/// Private Direct message
///
/// The expiration tag is relative to the gift wrap's `created_at`
/// so it doesn't leak the real send time.
/// Choose a `duration` greater than the 2 days
/// or the event may be created in an expired state.
///
/// <https://github.com/nostr-protocol/nips/blob/master/17.md>
#[uniffi::export(async_runtime = "tokio", default(expiration = None, rumor_extra_tags = []))]
pub async fn nip17_make_private_msg_async(
    signer: Arc<dyn AsyncNostrSigner>,
    receiver: &PublicKey,
    message: &str,
    expiration: Option<Duration>,
    rumor_extra_tags: Vec<Arc<Tag>>,
) -> Result<Event> {
    let signer = IntermediateAsyncNostrSigner::new(signer);
    let builder = make_builder(receiver, message, expiration, rumor_extra_tags);

    let event = builder.finalize_async(&signer).await?;
    Ok(event.into())
}

/// Extracts the relay list
///
/// <https://github.com/nostr-protocol/nips/blob/master/17.md>
#[uniffi::export]
pub fn nip17_extract_relay_list(event: &Event) -> Vec<Arc<RelayUrl>> {
    nip17::extract_relay_list(event.deref())
        .map(|u| Arc::new(u.clone().into()))
        .collect()
}
