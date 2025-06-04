// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use nostr_sdk::pool::policy;
use uniffi::Object;

use crate::error::Result;
use crate::protocol::event::Event;

#[derive(Debug, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct AdmitStatus {
    inner: policy::AdmitStatus,
}

impl Deref for AdmitStatus {
    type Target = policy::AdmitStatus;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[uniffi::export]
impl AdmitStatus {
    #[uniffi::constructor]
    pub fn success() -> Self {
        Self {
            inner: policy::AdmitStatus::Success,
        }
    }

    #[uniffi::constructor(default(reason = None))]
    pub fn rejected(reason: Option<String>) -> Self {
        Self {
            inner: policy::AdmitStatus::Rejected { reason },
        }
    }
}

// NOTE: for some reason the `#[uniffi::export(with_foreign)]` not allow to simply wrap `Result<Arc<T>>`, but wants a `Result<Option<Arc<T>>>`.
// TODO: when will be possible, remove the `Option` and keep just the `Result`.
#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait AdmitPolicy: Send + Sync {
    /// Admit connecting to a relay
    ///
    /// Returns `AdmitStatus`: `success` if the connection is allowed, otherwise `rejected`.
    async fn admit_connection(&self, relay_url: String) -> Result<Option<Arc<AdmitStatus>>>;

    /// Admit Event
    ///
    /// Returns `AdmitStatus`: `success` if the event is admitted, otherwise `rejected`.
    async fn admit_event(
        &self,
        relay_url: String,
        subscription_id: String,
        event: Arc<Event>,
    ) -> Result<Option<Arc<AdmitStatus>>>;
}

pub(crate) struct FFI2RustAdmitPolicy {
    pub(crate) inner: Arc<dyn AdmitPolicy>,
}

impl fmt::Debug for FFI2RustAdmitPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FFI2RustAdmitPolicy").finish()
    }
}

mod inner {
    use std::ops::Deref;
    use std::sync::Arc;

    use nostr::prelude::BoxedFuture;
    use nostr::{Event, RelayUrl, SubscriptionId};
    use nostr_sdk::pool::policy::AdmitPolicy;
    use nostr_sdk::prelude::{AdmitStatus, PolicyError};

    use super::FFI2RustAdmitPolicy;
    use crate::error::MiddleError;

    impl AdmitPolicy for FFI2RustAdmitPolicy {
        fn admit_connection<'a>(
            &'a self,
            relay_url: &'a RelayUrl,
        ) -> BoxedFuture<'a, Result<AdmitStatus, PolicyError>> {
            Box::pin(async move {
                let status = self
                    .inner
                    .admit_connection(relay_url.to_string())
                    .await
                    .map_err(MiddleError::from)
                    .map_err(PolicyError::backend)?;

                match status {
                    Some(s) => Ok(s.as_ref().deref().clone()),
                    None => Ok(AdmitStatus::rejected("Received a null admission status.")),
                }
            })
        }

        fn admit_event<'a>(
            &'a self,
            relay_url: &'a RelayUrl,
            subscription_id: &'a SubscriptionId,
            event: &'a Event,
        ) -> BoxedFuture<'a, Result<AdmitStatus, PolicyError>> {
            Box::pin(async move {
                let event = Arc::new(event.clone().into());
                let status = self
                    .inner
                    .admit_event(relay_url.to_string(), subscription_id.to_string(), event)
                    .await
                    .map_err(MiddleError::from)
                    .map_err(PolicyError::backend)?;

                match status {
                    Some(s) => Ok(s.as_ref().deref().clone()),
                    None => Ok(AdmitStatus::rejected("Received a null admission status.")),
                }
            })
        }
    }
}
