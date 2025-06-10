// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use std::fmt;
use std::sync::Arc;

use nostr_sdk::prelude;
use uniffi::Enum;

use super::SaveEventStatus;
use crate::error::Result;
use crate::protocol::event::{Event, EventId};
use crate::protocol::filter::Filter;

#[derive(Enum)]
pub enum DatabaseEventStatus {
    Saved,
    Deleted,
    NotExistent,
}

impl From<DatabaseEventStatus> for prelude::DatabaseEventStatus {
    fn from(value: DatabaseEventStatus) -> Self {
        match value {
            DatabaseEventStatus::Saved => Self::Saved,
            DatabaseEventStatus::Deleted => Self::Deleted,
            DatabaseEventStatus::NotExistent => Self::NotExistent,
        }
    }
}

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait CustomNostrDatabase: Send + Sync {
    /// Name of backend
    fn backend(&self) -> String;

    /// Save [`Event`] into store
    ///
    /// **This method assumes that [`Event`] was already verified**
    async fn save_event(&self, event: Arc<Event>) -> Result<Option<Arc<SaveEventStatus>>>;

    /// Check event status by ID
    ///
    /// Check if the event is saved, deleted or not existent.
    async fn check_id(&self, event_id: Arc<EventId>) -> Result<DatabaseEventStatus>;

    /// Get event by ID
    async fn event_by_id(&self, event_id: Arc<EventId>) -> Result<Option<Arc<Event>>>;

    /// Count the number of [`Event`] found by filter
    ///
    /// Use `Filter::new()` or `Filter::default()` to count all events.
    async fn count(&self, filters: Arc<Filter>) -> Result<u64>;

    /// Query store with filter
    async fn query(&self, filter: Arc<Filter>) -> Result<Vec<Arc<Event>>>;

    /// Delete all events that match the `Filter`
    async fn delete(&self, filter: Arc<Filter>) -> Result<()>;

    /// Wipe all data
    async fn wipe(&self) -> Result<()>;
}

pub(super) struct IntermediateCustomNostrDatabase {
    pub(super) inner: Arc<dyn CustomNostrDatabase>,
}

impl fmt::Debug for IntermediateCustomNostrDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IntermediateCustomNostrDatabase").finish()
    }
}

mod inner {
    use std::ops::Deref;
    use std::sync::Arc;

    use nostr_sdk::prelude::*;

    use super::IntermediateCustomNostrDatabase;
    use crate::error::MiddleError;

    impl NostrDatabase for IntermediateCustomNostrDatabase {
        fn backend(&self) -> Backend {
            Backend::Custom(self.inner.backend())
        }

        fn save_event<'a>(
            &'a self,
            event: &'a Event,
        ) -> BoxedFuture<'a, Result<SaveEventStatus, DatabaseError>> {
            Box::pin(async move {
                let status = self
                    .inner
                    .save_event(Arc::new(event.to_owned().into()))
                    .await
                    .map_err(|e| DatabaseError::backend(MiddleError::from(e)))?
                    .ok_or_else(|| {
                        DatabaseError::backend(MiddleError::new(
                            "Received null instead of SaveEventStatus",
                        ))
                    })?;
                Ok(status.inner)
            })
        }

        fn check_id<'a>(
            &'a self,
            event_id: &'a EventId,
        ) -> BoxedFuture<'a, Result<DatabaseEventStatus, DatabaseError>> {
            Box::pin(async move {
                self.inner
                    .check_id(Arc::new((*event_id).into()))
                    .await
                    .map(|s| s.into())
                    .map_err(|e| DatabaseError::backend(MiddleError::from(e)))
            })
        }

        fn event_by_id<'a>(
            &'a self,
            event_id: &'a EventId,
        ) -> BoxedFuture<'a, Result<Option<Event>, DatabaseError>> {
            Box::pin(async move {
                Ok(self
                    .inner
                    .event_by_id(Arc::new((*event_id).into()))
                    .await
                    .map_err(|e| DatabaseError::backend(MiddleError::from(e)))?
                    .map(|e| e.as_ref().deref().clone()))
            })
        }

        fn count(&self, filter: Filter) -> BoxedFuture<Result<usize, DatabaseError>> {
            Box::pin(async move {
                let res = self
                    .inner
                    .count(Arc::new(filter.into()))
                    .await
                    .map_err(|e| DatabaseError::backend(MiddleError::from(e)))?;
                Ok(res as usize)
            })
        }

        fn query(&self, filter: Filter) -> BoxedFuture<Result<Events, DatabaseError>> {
            Box::pin(async move {
                let mut events = Events::new(&filter);

                let output = self
                    .inner
                    .query(Arc::new(filter.into()))
                    .await
                    .map_err(|e| DatabaseError::backend(MiddleError::from(e)))?;

                // Extend events
                events.extend(output.into_iter().map(|e| e.as_ref().deref().clone()));

                Ok(events)
            })
        }

        fn delete(&self, filter: Filter) -> BoxedFuture<Result<(), DatabaseError>> {
            Box::pin(async move {
                self.inner
                    .delete(Arc::new(filter.into()))
                    .await
                    .map_err(|e| DatabaseError::backend(MiddleError::from(e)))
            })
        }

        fn wipe(&self) -> BoxedFuture<Result<(), DatabaseError>> {
            Box::pin(async move {
                self.inner
                    .wipe()
                    .await
                    .map_err(|e| DatabaseError::backend(MiddleError::from(e)))
            })
        }
    }
}
