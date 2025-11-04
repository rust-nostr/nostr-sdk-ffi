// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::borrow::Cow;
use std::ops::Deref;
use std::sync::Arc;

use nostr::Url;
use nostr::nips::nip22;
use uniffi::Enum;

use super::nip01::Coordinate;
use super::nip73::ExternalContentId;
use crate::error::NostrSdkError;
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
                id: Arc::new(id.into()),
                relay_hint: relay_hint.map(|u| Arc::new(u.into_owned().into())),
                pubkey_hint: pubkey_hint.map(|p| Arc::new(p.into())),
                kind: kind.map(|k| Arc::new(k.into())),
            },
            nip22::CommentTarget::Coordinate {
                address,
                relay_hint,
                ..
            } => Self::Address {
                address: Arc::new(address.into_owned().into()),
                relay_hint: relay_hint.map(|u| Arc::new(u.into_owned().into())),
            },
            nip22::CommentTarget::External { content, hint } => Self::External {
                content: content.into_owned().into(),
                hint: hint.map(|u| u.to_string()),
            },
        }
    }
}

impl TryFrom<CommentTarget> for nip22::CommentTarget<'static> {
    type Error = NostrSdkError;

    fn try_from(comment: CommentTarget) -> Result<Self, Self::Error> {
        match comment {
            CommentTarget::Event {
                id,
                relay_hint,
                pubkey_hint,
                kind,
            } => Ok(Self::Event {
                id: **id,
                relay_hint: relay_hint.map(|u| Cow::Owned(u.as_ref().deref().clone())),
                pubkey_hint: pubkey_hint.map(|p| **p),
                kind: kind.map(|k| **k),
            }),
            CommentTarget::Address {
                address,
                relay_hint,
                ..
            } => Ok(Self::Coordinate {
                address: Cow::Owned(address.as_ref().deref().clone()),
                relay_hint: relay_hint.map(|u| Cow::Owned(u.as_ref().deref().clone())),
                #[allow(deprecated)]
                kind: None,
            }),
            CommentTarget::External { content, hint } => Ok(Self::External {
                content: Cow::Owned(content.try_into()?),
                hint: match hint {
                    Some(hint) => Some(Cow::Owned(Url::parse(&hint)?)),
                    None => None,
                },
            }),
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
