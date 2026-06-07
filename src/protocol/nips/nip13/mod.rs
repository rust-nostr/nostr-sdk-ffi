// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::fmt;
use std::sync::Arc;

use nostr::nips::nip13;
use uniffi::Object;

mod macros;

pub(crate) use self::macros::*;
use crate::error::Result;
use crate::protocol::event::UnsignedEvent;

/// Gets the number of leading zero bits. Result is between 0 and 255.
#[uniffi::export]
pub fn get_leading_zero_bits(bytes: Vec<u8>) -> u8 {
    nip13::get_leading_zero_bits(bytes)
}

/// Returns all possible ID prefixes (hex) that have the specified number of leading zero bits.
///
/// Possible values: 0-255
#[uniffi::export]
pub fn get_prefixes_for_difficulty(leading_zero_bits: u8) -> Vec<String> {
    nip13::get_prefixes_for_difficulty(leading_zero_bits)
}

/// A single-threaded PoW miner implementation
#[derive(Object)]
pub struct SingleThreadPow {
    inner: nip13::SingleThreadPow,
}

#[uniffi::export]
impl SingleThreadPow {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            inner: nip13::SingleThreadPow,
        }
    }
}

export_pow_adapter!(SingleThreadPow, |adapter| &adapter.inner);
export_async_pow_adapter!(SingleThreadPow, |adapter| &adapter.inner);

/// A multithreaded Proof-of-Work miner.
#[derive(Object)]
#[cfg(feature = "pow-multi-thread")]
pub struct MultiThreadPow {
    inner: nip13::MultiThreadPow,
}

#[cfg(feature = "pow-multi-thread")]
#[uniffi::export]
impl MultiThreadPow {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            inner: nip13::MultiThreadPow,
        }
    }
}

#[cfg(feature = "pow-multi-thread")]
export_pow_adapter!(MultiThreadPow, |adapter| &adapter.inner);
#[cfg(feature = "pow-multi-thread")]
export_async_pow_adapter!(MultiThreadPow, |adapter| &adapter.inner);

/// A trait for custom Proof of Work computation.
#[uniffi::export(with_foreign)]
pub trait PowAdapter: Send + Sync {
    /// Computes Proof of Work for an unsigned event to meet the target
    /// difficulty.
    fn compute(&self, unsigned: Arc<UnsignedEvent>, difficulty: u8) -> Result<Arc<UnsignedEvent>>;
}

/// A trait for custom Proof of Work computation.
#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait AsyncPowAdapter: Send + Sync {
    /// Computes Proof of Work for an unsigned event to meet the target
    /// difficulty.
    async fn compute_async(
        &self,
        unsigned: Arc<UnsignedEvent>,
        difficulty: u8,
    ) -> Result<Option<Arc<UnsignedEvent>>>;
}

pub(crate) struct IntermediatePowAdapter {
    inner: Arc<dyn PowAdapter>,
}

impl fmt::Debug for IntermediatePowAdapter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IntermediatePowAdapter").finish()
    }
}

impl IntermediatePowAdapter {
    #[inline]
    pub(crate) fn new(inner: Arc<dyn PowAdapter>) -> Self {
        Self { inner }
    }
}

pub(crate) struct IntermediateAsyncPowAdapter {
    inner: Arc<dyn AsyncPowAdapter>,
}

impl fmt::Debug for IntermediateAsyncPowAdapter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IntermediateAsyncPowAdapter").finish()
    }
}

impl IntermediateAsyncPowAdapter {
    #[inline]
    pub(crate) fn new(inner: Arc<dyn AsyncPowAdapter>) -> Self {
        Self { inner }
    }
}

mod inner {
    use std::num::NonZeroU8;
    use std::ops::Deref;
    use std::sync::Arc;

    use async_trait::async_trait;
    use nostr::prelude::*;

    use super::{IntermediateAsyncPowAdapter, IntermediatePowAdapter};
    use crate::error::{MiddleError, NostrSdkError};

    impl PowAdapter for IntermediatePowAdapter {
        type Error = NostrSdkError;

        fn compute(
            &self,
            unsigned: UnsignedEvent,
            difficulty: NonZeroU8,
        ) -> Result<UnsignedEvent, Self::Error> {
            let unsigned = Arc::new(unsigned.into());
            let output = self
                .inner
                .compute(unsigned, difficulty.get())
                .map_err(MiddleError::from)?;
            Ok(output.as_ref().deref().clone())
        }
    }

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    impl AsyncPowAdapter for IntermediateAsyncPowAdapter {
        type Error = NostrSdkError;

        fn compute_async(
            &self,
            unsigned: UnsignedEvent,
            difficulty: NonZeroU8,
        ) -> BoxedFuture<'_, Result<UnsignedEvent, Self::Error>> {
            Box::pin(async move {
                let unsigned = Arc::new(unsigned.into());
                let output = self
                    .inner
                    .compute_async(unsigned, difficulty.get())
                    .await
                    .map_err(MiddleError::from)?
                    .ok_or_else(|| MiddleError::new("Received None instead of unsigned event"))?;
                Ok(output.as_ref().deref().clone())
            })
        }
    }
}
