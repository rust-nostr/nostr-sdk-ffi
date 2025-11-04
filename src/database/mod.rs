// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;
use std::sync::Arc;

#[cfg(feature = "lmdb")]
use nostr_lmdb::NostrLMDB;
#[cfg(feature = "ndb")]
use nostr_ndb::NdbDatabase;
use nostr_sdk::prelude::{self, IntoNostrDatabase, NostrDatabaseExt};
use uniffi::{Enum, Object};

pub mod custom;
pub mod events;

use self::custom::{CustomNostrDatabase, IntermediateCustomNostrDatabase};
use self::events::Events;
use crate::error::Result;
use crate::protocol::event::{Event, EventId};
use crate::protocol::filter::Filter;
use crate::protocol::key::PublicKey;
use crate::protocol::nips::nip01::Metadata;

/// Reason why event wasn't stored into the database
#[derive(Enum)]
pub enum RejectedReason {
    /// Ephemeral events aren't expected to be stored
    Ephemeral,
    /// The event already exists
    Duplicate,
    /// The event was deleted
    Deleted,
    /// The event is expired
    Expired,
    /// The event was replaced
    Replaced,
    /// Attempt to delete a non-owned event
    InvalidDelete,
    /// Other reason
    Other,
}

impl From<prelude::RejectedReason> for RejectedReason {
    fn from(status: prelude::RejectedReason) -> Self {
        match status {
            prelude::RejectedReason::Ephemeral => Self::Ephemeral,
            prelude::RejectedReason::Duplicate => Self::Duplicate,
            prelude::RejectedReason::Deleted => Self::Deleted,
            prelude::RejectedReason::Expired => Self::Expired,
            prelude::RejectedReason::Replaced => Self::Replaced,
            prelude::RejectedReason::InvalidDelete => Self::InvalidDelete,
            prelude::RejectedReason::Other => Self::Other,
        }
    }
}

impl From<RejectedReason> for prelude::RejectedReason {
    fn from(status: RejectedReason) -> Self {
        match status {
            RejectedReason::Ephemeral => Self::Ephemeral,
            RejectedReason::Duplicate => Self::Duplicate,
            RejectedReason::Deleted => Self::Deleted,
            RejectedReason::Expired => Self::Expired,
            RejectedReason::Replaced => Self::Replaced,
            RejectedReason::InvalidDelete => Self::InvalidDelete,
            RejectedReason::Other => Self::Other,
        }
    }
}

/// Save event status
#[derive(Object)]
pub struct SaveEventStatus {
    inner: prelude::SaveEventStatus,
}

impl From<prelude::SaveEventStatus> for SaveEventStatus {
    fn from(inner: prelude::SaveEventStatus) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl SaveEventStatus {
    #[uniffi::constructor]
    pub fn success() -> Self {
        Self {
            inner: prelude::SaveEventStatus::Success,
        }
    }

    #[uniffi::constructor]
    pub fn rejected(reason: RejectedReason) -> Self {
        Self {
            inner: prelude::SaveEventStatus::Rejected(reason.into()),
        }
    }

    /// The event has been successfully saved
    pub fn is_success(&self) -> bool {
        self.inner.is_success()
    }

    /// Get rejection reason, if the event wasn't saved successfully
    pub fn rejection_reason(&self) -> Option<RejectedReason> {
        match self.inner {
            prelude::SaveEventStatus::Success => None,
            prelude::SaveEventStatus::Rejected(reason) => Some(reason.into()),
        }
    }
}

#[derive(Object)]
pub struct NostrDatabase {
    inner: Arc<dyn prelude::NostrDatabase>,
}

impl Deref for NostrDatabase {
    type Target = Arc<dyn prelude::NostrDatabase>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<Arc<dyn prelude::NostrDatabase>> for NostrDatabase {
    fn from(inner: Arc<dyn prelude::NostrDatabase>) -> Self {
        Self { inner }
    }
}

#[cfg(feature = "lmdb")]
#[uniffi::export]
impl NostrDatabase {
    /// LMDB backend
    #[uniffi::constructor]
    pub fn lmdb(path: &str) -> Result<Self> {
        let db = Arc::new(NostrLMDB::open(path)?);
        Ok(Self {
            inner: db.into_nostr_database(),
        })
    }
}

#[cfg(feature = "ndb")]
#[uniffi::export]
impl NostrDatabase {
    /// [`nostrdb`](https://github.com/damus-io/nostrdb) backend
    #[uniffi::constructor]
    pub fn ndb(path: &str) -> Result<Self> {
        let db = Arc::new(NdbDatabase::open(path)?);
        Ok(Self {
            inner: db.into_nostr_database(),
        })
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl NostrDatabase {
    /// Open a custom nostr database
    #[uniffi::constructor]
    pub fn custom(database: Arc<dyn CustomNostrDatabase>) -> Self {
        let intermediate = IntermediateCustomNostrDatabase { inner: database };

        Self {
            inner: intermediate.into_nostr_database(),
        }
    }

    /// Save [`Event`] into store
    pub async fn save_event(&self, event: &Event) -> Result<SaveEventStatus> {
        Ok(self.inner.save_event(event.deref()).await?.into())
    }

    /// Get [`Event`] by [`EventId`]
    pub async fn event_by_id(&self, event_id: &EventId) -> Result<Option<Arc<Event>>> {
        Ok(self
            .inner
            .event_by_id(event_id.deref())
            .await?
            .map(|e| Arc::new(e.into())))
    }

    pub async fn count(&self, filter: &Filter) -> Result<u64> {
        Ok(self.inner.count(filter.deref().clone()).await? as u64)
    }

    pub async fn query(&self, filter: &Filter) -> Result<Arc<Events>> {
        Ok(Arc::new(
            self.inner.query(filter.deref().clone()).await?.into(),
        ))
    }

    /// Delete all events that match the `Filter`
    pub async fn delete(&self, filter: &Filter) -> Result<()> {
        Ok(self.inner.delete(filter.deref().clone()).await?)
    }

    /// Wipe all data
    pub async fn wipe(&self) -> Result<()> {
        Ok(self.inner.wipe().await?)
    }

    pub async fn metadata(&self, public_key: &PublicKey) -> Result<Option<Arc<Metadata>>> {
        Ok(self
            .inner
            .metadata(**public_key)
            .await?
            .map(|m| Arc::new(m.into())))
    }
}
