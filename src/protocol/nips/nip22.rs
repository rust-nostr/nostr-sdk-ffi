// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;
use std::sync::Arc;

use nostr::nips::nip22;
use uniffi::Enum;

use super::nip01::Coordinate;
use super::nip73::ExternalContentId;
use crate::protocol::event::{Event, EventId, Kind};
use crate::protocol::key::PublicKey;
use crate::protocol::types::RelayUrl;

/// Comment target
///
/// <https://github.com/nostr-protocol/nips/blob/master/22.md>
#[derive(Enum)]
pub enum CommentTarget {
    /// Event
    Event {
        /// Event ID
        id: Arc<EventId>,
        /// Relay hint
        relay_hint: Option<Arc<RelayUrl>>,
        /// Public key hint
        pubkey_hint: Option<Arc<PublicKey>>,
        /// Kind
        kind: Option<Arc<Kind>>,
    },
    /// Coordinate
    // NOTE: the enum variant can't have the same name of types in inner fields, otherwise will create issues with kotlin,
    // so rename this to `Address`.
    Address {
        /// Coordinate
        address: Arc<Coordinate>,
        /// Relay hint
        relay_hint: Option<Arc<RelayUrl>>,
        /// Kind
        kind: Option<Arc<Kind>>,
    },
    /// External content
    External {
        /// Content
        content: ExternalContentId,
        /// Web hint
        hint: Option<String>,
    },
}

impl From<nip22::CommentTarget<'_>> for CommentTarget {
    fn from(comment: nip22::CommentTarget<'_>) -> Self {
        match comment {
            nip22::CommentTarget::Event {
                id,
                relay_hint,
                pubkey_hint,
                kind,
            } => Self::Event {
                id: Arc::new((*id).into()),
                relay_hint: relay_hint.cloned().map(|u| Arc::new(u.into())),
                pubkey_hint: pubkey_hint.map(|p| Arc::new((*p).into())),
                kind: kind.map(|k| Arc::new((*k).into())),
            },
            nip22::CommentTarget::Coordinate {
                address,
                relay_hint,
                kind,
            } => Self::Address {
                address: Arc::new(address.clone().into()),
                relay_hint: relay_hint.cloned().map(|u| Arc::new(u.into())),
                kind: kind.map(|k| Arc::new((*k).into())),
            },
            nip22::CommentTarget::External { content, hint } => Self::External {
                content: content.clone().into(),
                hint: hint.map(|u| u.to_string()),
            },
        }
    }
}

/// Extract NIP22 root comment target
#[uniffi::export]
pub fn nip22_extract_root(event: &Event) -> Option<CommentTarget> {
    nip22::extract_root(event.deref()).map(|c| c.into())
}

/// Extract NIP22 parent comment target
#[uniffi::export]
pub fn nip22_extract_parent(event: &Event) -> Option<CommentTarget> {
    nip22::extract_parent(event.deref()).map(|c| c.into())
}
