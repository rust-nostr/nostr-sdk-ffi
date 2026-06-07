// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::fmt;
use std::sync::Arc;

mod macros;

pub(crate) use self::macros::*;
use super::event::{Event, UnsignedEvent};
use super::key::PublicKey;
use crate::error::Result;

#[uniffi::export(with_foreign)]
pub trait NostrSigner: Send + Sync {
    /// Get signer public key
    fn get_public_key(&self) -> Result<Arc<PublicKey>>;

    /// Sign an unsigned event
    fn sign_event(&self, unsigned_event: Arc<UnsignedEvent>) -> Result<Arc<Event>>;

    /// NIP04 encrypt (deprecate and unsecure)
    fn nip04_encrypt(&self, public_key: Arc<PublicKey>, content: String) -> Result<String>;

    /// NIP04 decrypt
    fn nip04_decrypt(
        &self,
        public_key: Arc<PublicKey>,
        encrypted_content: String,
    ) -> Result<String>;

    /// NIP44 encrypt
    fn nip44_encrypt(&self, public_key: Arc<PublicKey>, content: String) -> Result<String>;

    /// NIP44 decrypt
    fn nip44_decrypt(&self, public_key: Arc<PublicKey>, payload: String) -> Result<String>;
}

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait AsyncNostrSigner: Send + Sync {
    /// Get signer public key
    async fn get_public_key_async(&self) -> Result<Option<Arc<PublicKey>>>;

    /// Sign an unsigned event
    async fn sign_event_async(
        &self,
        unsigned_event: Arc<UnsignedEvent>,
    ) -> Result<Option<Arc<Event>>>;

    /// NIP04 encrypt (deprecate and unsecure)
    async fn nip04_encrypt_async(
        &self,
        public_key: Arc<PublicKey>,
        content: String,
    ) -> Result<String>;

    /// NIP04 decrypt
    async fn nip04_decrypt_async(
        &self,
        public_key: Arc<PublicKey>,
        encrypted_content: String,
    ) -> Result<String>;

    /// NIP44 encrypt
    async fn nip44_encrypt_async(
        &self,
        public_key: Arc<PublicKey>,
        content: String,
    ) -> Result<String>;

    /// NIP44 decrypt
    async fn nip44_decrypt_async(
        &self,
        public_key: Arc<PublicKey>,
        payload: String,
    ) -> Result<String>;
}

pub(crate) struct IntermediateNostrSigner {
    pub(super) inner: Arc<dyn NostrSigner>,
}

impl fmt::Debug for IntermediateNostrSigner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IntermediateNostrSigner").finish()
    }
}

impl IntermediateNostrSigner {
    #[inline]
    pub(crate) fn new(inner: Arc<dyn NostrSigner>) -> Self {
        Self { inner }
    }
}

pub(crate) struct IntermediateAsyncNostrSigner {
    pub(super) inner: Arc<dyn AsyncNostrSigner>,
}

impl fmt::Debug for IntermediateAsyncNostrSigner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IntermediateAsyncNostrSigner").finish()
    }
}

impl IntermediateAsyncNostrSigner {
    #[inline]
    pub(crate) fn new(inner: Arc<dyn AsyncNostrSigner>) -> Self {
        Self { inner }
    }
}

mod inner {
    use std::ops::Deref;
    use std::sync::Arc;

    use async_trait::async_trait;
    use nostr::prelude::*;

    use super::{IntermediateAsyncNostrSigner, IntermediateNostrSigner};
    use crate::error::MiddleError;

    impl GetPublicKey for IntermediateNostrSigner {
        type Error = SignerError;

        fn get_public_key(&self) -> Result<PublicKey, Self::Error> {
            let public_key = self
                .inner
                .get_public_key()
                .map_err(|e| SignerError::backend(MiddleError::from(e)))?;
            Ok(**public_key)
        }
    }

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    impl AsyncGetPublicKey for IntermediateAsyncNostrSigner {
        type Error = SignerError;

        fn get_public_key_async(&self) -> BoxedFuture<Result<PublicKey, Self::Error>> {
            Box::pin(async move {
                let public_key = self
                    .inner
                    .get_public_key_async()
                    .await
                    .map_err(|e| SignerError::backend(MiddleError::from(e)))?
                    .ok_or_else(|| {
                        SignerError::backend(MiddleError::new(
                            "Received None instead of public key",
                        ))
                    })?;
                Ok(**public_key)
            })
        }
    }

    #[async_trait]
    impl SignEvent for IntermediateNostrSigner {
        type Error = SignerError;

        fn sign_event(&self, unsigned: UnsignedEvent) -> Result<Event, SignerError> {
            let unsigned = Arc::new(unsigned.into());
            let event = self
                .inner
                .sign_event(unsigned)
                .map_err(|e| SignerError::backend(MiddleError::from(e)))?;
            Ok(event.as_ref().deref().clone())
        }
    }

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    impl AsyncSignEvent for IntermediateAsyncNostrSigner {
        type Error = SignerError;

        fn sign_event_async(
            &self,
            unsigned: UnsignedEvent,
        ) -> BoxedFuture<Result<Event, Self::Error>> {
            Box::pin(async move {
                let unsigned = Arc::new(unsigned.into());
                let event = self
                    .inner
                    .sign_event_async(unsigned)
                    .await
                    .map_err(|e| SignerError::backend(MiddleError::from(e)))?
                    .ok_or_else(|| {
                        SignerError::backend(MiddleError::new("Received None instead of event"))
                    })?;
                Ok(event.as_ref().deref().clone())
            })
        }
    }

    #[async_trait]
    impl Nip04 for IntermediateNostrSigner {
        type Error = SignerError;

        fn nip04_encrypt(
            &self,
            public_key: &PublicKey,
            content: &str,
        ) -> Result<String, Self::Error> {
            let public_key = Arc::new((*public_key).into());
            self.inner
                .nip04_encrypt(public_key, content.to_string())
                .map_err(|e| SignerError::backend(MiddleError::from(e)))
        }

        fn nip04_decrypt(
            &self,
            public_key: &PublicKey,
            content: &str,
        ) -> Result<String, Self::Error> {
            let public_key = Arc::new((*public_key).into());
            self.inner
                .nip04_decrypt(public_key, content.to_string())
                .map_err(|e| SignerError::backend(MiddleError::from(e)))
        }
    }

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    impl AsyncNip04 for IntermediateAsyncNostrSigner {
        type Error = SignerError;

        fn nip04_encrypt_async<'a>(
            &'a self,
            public_key: &'a PublicKey,
            content: &'a str,
        ) -> BoxedFuture<'a, Result<String, Self::Error>> {
            Box::pin(async move {
                let public_key = Arc::new((*public_key).into());
                self.inner
                    .nip04_encrypt_async(public_key, content.to_string())
                    .await
                    .map_err(|e| SignerError::backend(MiddleError::from(e)))
            })
        }

        fn nip04_decrypt_async<'a>(
            &'a self,
            public_key: &'a PublicKey,
            content: &'a str,
        ) -> BoxedFuture<'a, Result<String, Self::Error>> {
            Box::pin(async move {
                let public_key = Arc::new((*public_key).into());
                self.inner
                    .nip04_decrypt_async(public_key, content.to_string())
                    .await
                    .map_err(|e| SignerError::backend(MiddleError::from(e)))
            })
        }
    }

    #[async_trait]
    impl Nip44 for IntermediateNostrSigner {
        type Error = SignerError;

        fn nip44_encrypt(
            &self,
            public_key: &PublicKey,
            content: &str,
        ) -> Result<String, Self::Error> {
            let public_key = Arc::new((*public_key).into());
            self.inner
                .nip44_encrypt(public_key, content.to_string())
                .map_err(|e| SignerError::backend(MiddleError::from(e)))
        }

        fn nip44_decrypt(
            &self,
            public_key: &PublicKey,
            content: &str,
        ) -> Result<String, Self::Error> {
            let public_key = Arc::new((*public_key).into());
            self.inner
                .nip44_decrypt(public_key, content.to_string())
                .map_err(|e| SignerError::backend(MiddleError::from(e)))
        }
    }

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    impl AsyncNip44 for IntermediateAsyncNostrSigner {
        type Error = SignerError;

        fn nip44_encrypt_async<'a>(
            &'a self,
            public_key: &'a PublicKey,
            content: &'a str,
        ) -> BoxedFuture<'a, Result<String, Self::Error>> {
            Box::pin(async move {
                let public_key = Arc::new((*public_key).into());
                self.inner
                    .nip44_encrypt_async(public_key, content.to_string())
                    .await
                    .map_err(|e| SignerError::backend(MiddleError::from(e)))
            })
        }

        fn nip44_decrypt_async<'a>(
            &'a self,
            public_key: &'a PublicKey,
            content: &'a str,
        ) -> BoxedFuture<'a, Result<String, Self::Error>> {
            Box::pin(async move {
                let public_key = Arc::new((*public_key).into());
                self.inner
                    .nip44_decrypt_async(public_key, content.to_string())
                    .await
                    .map_err(|e| SignerError::backend(MiddleError::from(e)))
            })
        }
    }
}
