// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;
use std::sync::Arc;

use nostr::event::tag;
use nostr::prelude::{Nip89Tag, TagCodec};
use uniffi::Object;

pub mod list;

pub use self::list::Tags;
use crate::error::Result;
use crate::protocol::event::{EventId, PublicKey};
use crate::protocol::filter::SingleLetterTag;
use crate::protocol::nips::nip01::Coordinate;
use crate::protocol::types::{RelayUrl, Timestamp};

/// Tag
#[derive(Debug, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct Tag {
    inner: tag::Tag,
}

impl Deref for Tag {
    type Target = tag::Tag;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<tag::Tag> for Tag {
    fn from(inner: tag::Tag) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl Tag {
    /// Parse tag
    ///
    /// Return error if the tag is empty!
    #[uniffi::constructor]
    pub fn parse(data: Vec<String>) -> Result<Self> {
        Ok(Self {
            inner: tag::Tag::parse(data)?,
        })
    }

    /// Get tag kind
    pub fn kind(&self) -> String {
        self.inner.kind().to_string()
    }

    /// Return the **first** tag value (index `1`), if exists.
    pub fn content(&self) -> Option<String> {
        self.inner.content().map(|c| c.to_string())
    }

    /// Get `SingleLetterTag`
    pub fn single_letter_tag(&self) -> Option<Arc<SingleLetterTag>> {
        self.inner.single_letter_tag().map(|s| Arc::new(s.into()))
    }

    // /// Get standardized tag
    // pub fn as_standardized(&self) -> Option<TagStandard> {
    //     self.inner.as_standardized().cloned().map(|t| t.into())
    // }

    /// Get tag len
    pub fn len(&self) -> u64 {
        self.inner.len() as u64
    }

    /// Get array of strings
    pub fn as_vec(&self) -> Vec<String> {
        self.inner.as_slice().to_vec()
    }

    /// Compose `["e", "<event-id">]`
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/01.md>
    #[uniffi::constructor]
    pub fn event(event_id: &EventId) -> Self {
        Self {
            inner: tag::Tag::event(**event_id),
        }
    }

    /// Compose `["p", "<public-key>"]` tag
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/01.md>
    #[uniffi::constructor]
    pub fn public_key(public_key: &PublicKey) -> Self {
        Self {
            inner: tag::Tag::public_key(**public_key),
        }
    }

    /// Compose `["d", "<identifier>"]` tag
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/01.md>
    #[uniffi::constructor]
    pub fn identifier(identifier: &str) -> Self {
        Self {
            inner: tag::Tag::identifier(identifier),
        }
    }

    /// Compose `["a", "<coordinate>"]` tag
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/01.md>
    #[uniffi::constructor(default(relay_url = None))]
    pub fn coordinate(coordinate: &Coordinate, relay_url: Option<Arc<RelayUrl>>) -> Self {
        let relay_url: Option<nostr::RelayUrl> = relay_url.map(|u| u.as_ref().deref().clone());
        Self {
            inner: tag::Tag::coordinate(coordinate.deref().clone(), relay_url),
        }
    }

    /// Compose `["nonce", "<nonce>", "<difficulty>"]` tag
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/13.md>
    #[uniffi::constructor]
    pub fn pow(nonce: u64, difficulty: u8) -> Self {
        Self {
            inner: tag::Tag::pow(nonce as u128, difficulty),
        }
    }

    /// Construct `["client", "<name>"]` tag
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/89.md>
    #[uniffi::constructor]
    pub fn client(name: String) -> Self {
        Self {
            inner: Nip89Tag::Client {
                name,
                address: None,
            }
            .to_tag(),
        }
    }

    /// Compose `["expiration", "<timestamp>"]` tag
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/40.md>
    #[uniffi::constructor]
    pub fn expiration(timestamp: &Timestamp) -> Self {
        Self {
            inner: tag::Tag::expiration(**timestamp),
        }
    }

    /// Compose `["t", "<hashtag>"]` tag
    ///
    /// This will convert the hashtag to lowercase.
    #[uniffi::constructor]
    pub fn hashtag(hashtag: &str) -> Self {
        Self {
            inner: tag::Tag::hashtag(hashtag),
        }
    }

    /// Protected event
    ///
    /// JSON: `["-"]`
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/70.md>
    #[uniffi::constructor]
    pub fn protected() -> Self {
        Self {
            inner: tag::Tag::protected(),
        }
    }

    /// A short human-readable plaintext summary of what that event is about
    ///
    /// JSON: `["alt", "<summary>"]`
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/31.md>
    #[uniffi::constructor]
    pub fn alt(summary: &str) -> Self {
        Self {
            inner: tag::Tag::alt(summary),
        }
    }

    /// Compose custom tag
    ///
    /// JSON: `["<kind>", "<value-1>", "<value-2>", ...]`
    #[uniffi::constructor]
    pub fn custom(kind: &str, values: &[String]) -> Self {
        Self {
            inner: tag::Tag::custom(kind, values),
        }
    }

    /// Check if it's a protected event tag
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/70.md>
    pub fn is_protected(&self) -> bool {
        self.inner.is_protected()
    }
}
