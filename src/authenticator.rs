// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use nostr_sdk::authenticator::{self, Authenticator as _};
use uniffi::Object;

use crate::error::{NostrSdkError, Result};
use crate::protocol::event::Event;
use crate::protocol::signer::{AsyncNostrSigner, IntermediateAsyncNostrSigner};
use crate::protocol::types::RelayUrl;

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait Authenticator: Send + Sync {
    /// Make a NIP-42 authentication event.
    async fn make_auth_event(
        &self,
        relay_url: Arc<RelayUrl>,
        challenge: String,
    ) -> Result<Option<Arc<Event>>>;
}

pub(crate) struct FFI2RustAuthenticator {
    pub(crate) inner: Arc<dyn Authenticator>,
}

impl fmt::Debug for FFI2RustAuthenticator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FFI2RustAuthenticator").finish()
    }
}

/// A wrapper around an async signer that implements the `Authenticator`.
#[derive(Object)]
pub struct SignerAuthenticator {
    signer: authenticator::SignerAuthenticator<IntermediateAsyncNostrSigner>,
}

#[uniffi::export]
impl SignerAuthenticator {
    /// Construct an authenticator from an async signer.
    #[inline]
    #[uniffi::constructor]
    pub fn new(signer: Arc<dyn AsyncNostrSigner>) -> Self {
        Self {
            signer: authenticator::SignerAuthenticator::new(IntermediateAsyncNostrSigner::new(
                signer,
            )),
        }
    }
}

#[uniffi::export]
#[async_trait::async_trait]
impl Authenticator for SignerAuthenticator {
    async fn make_auth_event(
        &self,
        relay_url: Arc<RelayUrl>,
        challenge: String,
    ) -> Result<Option<Arc<Event>>> {
        let event = crate::future::assume_send(
            self.signer
                .make_auth_event(relay_url.as_ref().deref(), &challenge),
        )
        .await
        .map_err(|e| NostrSdkError::Generic(e.to_string()))?;
        Ok(Some(Arc::new(event.into())))
    }
}

mod inner {
    use std::ops::Deref;
    use std::sync::Arc;

    use nostr::prelude::BoxedFuture;
    use nostr::{Event, RelayUrl};
    use nostr_sdk::authenticator::Authenticator;
    use nostr_sdk::error::Error;

    use super::FFI2RustAuthenticator;
    use crate::error::MiddleError;

    impl Authenticator for FFI2RustAuthenticator {
        fn make_auth_event<'a>(
            &'a self,
            relay_url: &'a RelayUrl,
            challenge: &'a str,
        ) -> BoxedFuture<'a, Result<Event, Error>> {
            Box::pin(async move {
                let event = self
                    .inner
                    .make_auth_event(Arc::new(relay_url.clone().into()), challenge.to_string())
                    .await
                    .map_err(MiddleError::from)
                    .map_err(Error::other)?;

                match event {
                    Some(event) => Ok(event.as_ref().deref().clone()),
                    None => Err(Error::other(MiddleError::new(
                        "Received a null authentication event.",
                    ))),
                }
            })
        }
    }
}
