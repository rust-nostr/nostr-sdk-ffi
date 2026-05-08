// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;
use std::sync::Arc;

use nostr::nips::nip17;

use crate::error::Result;
use crate::protocol::event::{Event, Tag};
use crate::protocol::key::PublicKey;
use crate::protocol::signer::NostrSigner;
use crate::protocol::types::RelayUrl;

/// Private Direct message
///
/// <https://github.com/nostr-protocol/nips/blob/master/17.md>
#[uniffi::export(async_runtime = "tokio", default(rumor_extra_tags = []))]
pub async fn make_private_msg(
    signer: &NostrSigner,
    receiver: &PublicKey,
    message: &str,
    rumor_extra_tags: Vec<Arc<Tag>>,
) -> Result<Event> {
    Ok(nostr::EventBuilder::private_msg(
        signer.deref(),
        **receiver,
        message,
        rumor_extra_tags
            .into_iter()
            .map(|t| t.as_ref().deref().clone()),
    )
    .await?
    .into())
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
