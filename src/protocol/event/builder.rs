// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

use nostr::event::{
    EventBuilderTemplate, FinalizeEvent, FinalizeEventAsync, FinalizeUnsignedEvent,
};
use nostr::nips::{nip02, nip22};
use uniffi::Object;

use super::{Event, EventId, Kind};
use crate::error::{NostrSdkError, Result};
use crate::protocol::event::{PublicKey, Tag, Timestamp, UnsignedEvent};
use crate::protocol::nips::nip01::Metadata;
use crate::protocol::nips::nip09::EventDeletionRequest;
use crate::protocol::nips::nip22::CommentTarget;
use crate::protocol::nips::nip34::{GitIssue, GitPatch, GitRepositoryAnnouncement};
use crate::protocol::nips::nip65::RelayMetadata;
use crate::protocol::nips::nip90::JobFeedbackData;
use crate::protocol::signer::{
    AsyncNostrSigner, IntermediateAsyncNostrSigner, IntermediateNostrSigner, NostrSigner,
};
use crate::protocol::types::{Contact, RelayUrl};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct EventBuilder {
    inner: nostr::EventBuilder,
}

impl From<nostr::EventBuilder> for EventBuilder {
    fn from(inner: nostr::EventBuilder) -> Self {
        Self { inner }
    }
}

impl Deref for EventBuilder {
    type Target = nostr::EventBuilder;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl EventBuilder {
    #[uniffi::constructor]
    pub fn new(kind: &Kind, content: &str) -> Self {
        Self {
            inner: nostr::EventBuilder::new(**kind, content),
        }
    }

    /// Add tags
    ///
    /// This method extend the current tags (if any).
    pub fn tags(&self, tags: &[Arc<Tag>]) -> Self {
        let mut builder = self.clone();
        let tags = tags.iter().map(|t| t.as_ref().deref().clone());
        builder.inner = builder.inner.tags(tags);
        builder
    }

    /// Set a custom `created_at` UNIX timestamp
    pub fn custom_created_at(&self, created_at: &Timestamp) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.custom_created_at(**created_at);
        builder
    }

    /// Build an unsigned event
    ///
    /// By default, this method removes any `p` tags that match the author's public key.
    /// To allow self-tagging, call [`EventBuilder::allow_self_tagging`] first.
    pub fn finalize_unsigned(&self, public_key: &PublicKey) -> UnsignedEvent {
        self.inner.clone().finalize_unsigned(**public_key).into()
    }

    /// Build, sign and return [`Event`]
    ///
    /// Check [`EventBuilder::build`] to learn more.
    pub fn finalize(&self, signer: Arc<dyn NostrSigner>) -> Result<Event> {
        let signer = IntermediateNostrSigner::new(signer);
        let event = self.inner.clone().finalize(&signer)?;
        Ok(event.into())
    }

    /// Build, sign and return [`Event`]
    ///
    /// Check [`EventBuilder::build`] to learn more.
    pub async fn finalize_async(&self, signer: Arc<dyn AsyncNostrSigner>) -> Result<Event> {
        let signer = IntermediateAsyncNostrSigner::new(signer);
        let event = self.inner.clone().finalize_async(&signer).await?;
        Ok(event.into())
    }

    /// Profile metadata
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/01.md>
    #[uniffi::constructor]
    pub fn metadata(metadata: &Metadata) -> Self {
        Self {
            #[allow(deprecated)]
            inner: nostr::EventBuilder::metadata(metadata.deref()),
        }
    }

    /// Relay list metadata
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/65.md>
    #[uniffi::constructor]
    pub fn relay_list(map: HashMap<Arc<RelayUrl>, Option<RelayMetadata>>) -> Result<Self> {
        let mut list = Vec::with_capacity(map.len());
        for (url, metadata) in map.into_iter() {
            let metadata = metadata.map(|m| m.into());
            list.push((url.as_ref().deref().clone(), metadata))
        }
        Ok(Self {
            inner: nostr::EventBuilder::relay_list(list),
        })
    }

    /// Text note
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/01.md>
    #[uniffi::constructor]
    pub fn text_note(content: &str) -> Self {
        Self {
            inner: nostr::EventBuilder::text_note(content),
        }
    }

    /// Text note reply
    ///
    /// This adds only the most significant tags, like:
    /// - `p` tag with the author of the `reply_to` and `root` events;
    /// - `e` tag of the `reply_to` and `root` events.
    ///
    /// Any additional necessary tag can be added with [`EventBuilder::tag`] or [`EventBuilder::tags`].
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/10.md>
    #[uniffi::constructor(default(root = None, relay_url = None))]
    pub fn text_note_reply(
        content: String,
        reply_to: &Event,
        root: Option<Arc<Event>>,
        relay_url: Option<Arc<RelayUrl>>,
    ) -> Result<Self> {
        Ok(Self {
            inner: nostr::EventBuilder::text_note_reply(
                content,
                reply_to.deref(),
                root.as_ref().map(|e| e.as_ref().deref()),
                relay_url.map(|u| u.as_ref().deref().clone()),
            ),
        })
    }

    /// Comment
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/22.md>
    #[uniffi::constructor(default(root = None))]
    pub fn comment(
        content: String,
        comment_to: CommentTarget,
        root: Option<CommentTarget>,
    ) -> Result<Self> {
        let comment_to: nip22::CommentTarget = comment_to.try_into()?;
        Ok(Self {
            inner: nostr::EventBuilder::comment(
                content,
                comment_to,
                match root {
                    Some(root) => Some(root.try_into()?),
                    None => None,
                },
            ),
        })
    }

    /// Long-form text note (generally referred to as "articles" or "blog posts").
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/23.md>
    #[uniffi::constructor]
    pub fn long_form_text_note(content: &str) -> Self {
        Self {
            inner: nostr::EventBuilder::long_form_text_note(content),
        }
    }

    /// Contact/Follow list
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/02.md>
    #[uniffi::constructor]
    pub fn contact_list(contacts: Vec<Contact>) -> Self {
        Self {
            inner: nip02::ContactListBuilder::new(contacts.into_iter().map(|c| c.into())).build(),
        }
    }

    /// Repost
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/18.md>
    #[uniffi::constructor(default(relay_url = None))]
    pub fn repost(event: &Event, relay_url: Option<Arc<RelayUrl>>) -> Result<Self> {
        Ok(Self {
            inner: nostr::EventBuilder::repost(
                event.deref(),
                relay_url.map(|u| u.as_ref().deref().clone()),
            ),
        })
    }

    /// Event deletion request
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/09.md>
    #[uniffi::constructor]
    pub fn delete(request: EventDeletionRequest) -> Self {
        Self {
            #[allow(deprecated)]
            inner: nostr::EventBuilder::delete(request.into()),
        }
    }

    /// Add reaction (like/upvote, dislike/downvote or emoji) to an event
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/25.md>
    #[uniffi::constructor]
    pub fn reaction(event: &Event, reaction: &str) -> Self {
        Self {
            inner: nostr::EventBuilder::reaction(event.deref(), reaction),
        }
    }

    /// Create new channel
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/28.md>
    #[uniffi::constructor]
    pub fn channel(metadata: &Metadata) -> Self {
        Self {
            inner: nostr::EventBuilder::channel(metadata.deref()),
        }
    }

    /// Channel metadata
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/28.md>
    #[uniffi::constructor(default(relay_url = None))]
    pub fn channel_metadata(
        channel_id: &EventId,
        metadata: &Metadata,
        relay_url: Option<Arc<RelayUrl>>,
    ) -> Result<Self> {
        Ok(Self {
            inner: nostr::EventBuilder::channel_metadata(
                **channel_id,
                relay_url.map(|u| u.as_ref().deref().clone()),
                metadata.deref(),
            ),
        })
    }

    /// Channel message
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/28.md>
    #[uniffi::constructor]
    pub fn channel_msg(channel_id: &EventId, relay_url: &RelayUrl, content: &str) -> Result<Self> {
        Ok(Self {
            inner: nostr::EventBuilder::channel_msg(
                **channel_id,
                relay_url.deref().clone(),
                content,
            ),
        })
    }

    /// Hide message
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/28.md>
    #[uniffi::constructor(default(reason = None))]
    pub fn hide_channel_msg(message_id: &EventId, reason: Option<String>) -> Self {
        Self {
            inner: nostr::EventBuilder::hide_channel_msg(**message_id, reason),
        }
    }

    /// Mute channel user
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/28.md>
    #[uniffi::constructor(default(reason = None))]
    pub fn mute_channel_user(public_key: &PublicKey, reason: Option<String>) -> Self {
        Self {
            inner: nostr::EventBuilder::mute_channel_user(**public_key, reason),
        }
    }

    /// Authentication of clients to relays
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/42.md>
    #[uniffi::constructor]
    pub fn auth(challenge: &str, relay_url: &RelayUrl) -> Result<Self> {
        Ok(Self {
            inner: nostr::EventBuilder::auth(challenge, relay_url.deref().clone()),
        })
    }

    /// Reporting
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/56.md>
    #[uniffi::constructor]
    pub fn report(tags: &[Arc<Tag>], content: &str) -> Self {
        let tags = tags.iter().map(|t| t.as_ref().deref().clone());
        Self {
            inner: nostr::EventBuilder::report(tags, content),
        }
    }

    /// Data Vending Machine (DVM) - Job Request
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/90.md>
    #[uniffi::constructor]
    pub fn job_request(kind: &Kind) -> Result<Self> {
        Ok(Self {
            inner: nostr::EventBuilder::job_request(**kind)
                .map_err(|e| NostrSdkError::Generic(e.to_string()))?,
        })
    }

    /// Data Vending Machine (DVM) - Job Result
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/90.md>
    #[uniffi::constructor(default(bolt11 = None))]
    pub fn job_result(
        job_request: &Event,
        payload: String,
        millisats: u64,
        bolt11: Option<String>,
    ) -> Result<Self> {
        Ok(Self {
            inner: nostr::EventBuilder::job_result(
                job_request.deref().clone(),
                payload,
                millisats,
                bolt11,
            )
            .map_err(|e| NostrSdkError::Generic(e.to_string()))?,
        })
    }

    /// Data Vending Machine (DVM) - Job Feedback
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/90.md>
    #[uniffi::constructor]
    pub fn job_feedback(data: &JobFeedbackData) -> Self {
        Self {
            inner: nostr::EventBuilder::job_feedback(data.deref().clone()),
        }
    }

    /// Label
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/32.md>
    #[uniffi::constructor]
    pub fn label(label_namespace: String, label: String) -> Self {
        Self {
            inner: nostr::EventBuilder::label(label_namespace, label),
        }
    }

    /// Git Repository Announcement
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/34.md>
    #[uniffi::constructor]
    pub fn git_repository_announcement(data: GitRepositoryAnnouncement) -> Result<Self> {
        Ok(Self {
            inner: nostr::EventBuilder::git_repository_announcement(data.into()),
        })
    }

    /// Git Issue
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/34.md>
    #[uniffi::constructor]
    pub fn git_issue(issue: GitIssue) -> Result<Self> {
        Ok(Self {
            inner: nostr::EventBuilder::git_issue(issue.into())?,
        })
    }

    /// Git Patch
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/34.md>
    #[uniffi::constructor]
    pub fn git_patch(patch: GitPatch) -> Result<Self> {
        Ok(Self {
            inner: nostr::EventBuilder::git_patch(patch.try_into()?)?,
        })
    }

    /// Private direct message relay list
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/17.md>
    #[uniffi::constructor]
    pub fn nip17_relay_list(urls: Vec<Arc<RelayUrl>>) -> Self {
        Self {
            inner: nostr::EventBuilder::nip17_relay_list(
                urls.into_iter().map(|r| r.as_ref().deref().clone()),
            ),
        }
    }
}
