macro_rules! impl_pow_adapter {
    ($type:ty, |$adapter:ident| $inner:expr) => {
        impl_pow_adapter!(
            @impl
            []
            $crate::protocol::nips::nip13::PowAdapter,
            $type,
            |$adapter| $inner
        );
    };

    (@impl [$($attr:tt)*] $trait:path, $type:ty, |$adapter:ident| $inner:expr) => {
        $($attr)*
        impl $trait for $type {
            fn compute(
                &self,
                unsigned: ::std::sync::Arc<$crate::protocol::event::UnsignedEvent>,
                difficulty: u8,
            ) -> $crate::error::Result<::std::sync::Arc<$crate::protocol::event::UnsignedEvent>> {
                use std::ops::Deref;

                let $adapter = self;
                let adapter = $inner;
                let unsigned = unsigned.as_ref().deref().clone();
                let difficulty = ::std::num::NonZeroU8::new(difficulty).ok_or($crate::error::NostrSdkError::NonZeroDifficulty)?;

                Ok(::std::sync::Arc::new(
                    ::nostr::nips::nip13::PowAdapter::compute(adapter, unsigned, difficulty)?.into(),
                ))
            }
        }
    };
}

macro_rules! export_pow_adapter {
    ($type:ty, |$adapter:ident| $inner:expr) => {
        $crate::protocol::nips::nip13::impl_pow_adapter!(
            @impl
            [#[uniffi::export]]
            PowAdapter,
            $type,
            |$adapter| $inner
        );
    };
}

macro_rules! impl_async_pow_adapter {
    ($type:ty, |$adapter:ident| $inner:expr) => {
        impl_async_pow_adapter!(
            @impl
            []
            $crate::protocol::nips::nip13::AsyncPowAdapter,
            $type,
            |$adapter| $inner
        );
    };

    (@impl [$($attr:tt)*] $trait:path, $type:ty, |$adapter:ident| $inner:expr) => {
        $($attr)*
        #[async_trait::async_trait]
        impl $trait for $type {
            async fn compute_async(
                &self,
                unsigned: ::std::sync::Arc<$crate::protocol::event::UnsignedEvent>,
                difficulty: u8,
            ) -> $crate::error::Result<Option<::std::sync::Arc<$crate::protocol::event::UnsignedEvent>>> {
                use std::ops::Deref;

                let $adapter = self;
                let adapter = $inner;
                let unsigned = unsigned.as_ref().deref().clone();
                let difficulty = ::std::num::NonZeroU8::new(difficulty).ok_or($crate::error::NostrSdkError::NonZeroDifficulty)?;

                Ok(Some(::std::sync::Arc::new(
                    $crate::future::assume_send(<_ as ::nostr::nips::nip13::AsyncPowAdapter>::compute_async(adapter, unsigned, difficulty)).await?.into(),
                )))
            }
        }
    };
}

macro_rules! export_async_pow_adapter {
    ($type:ty, |$adapter:ident| $inner:expr) => {
        $crate::protocol::nips::nip13::impl_async_pow_adapter!(
            @impl
            [#[uniffi::export(async_runtime = "tokio")]]
            AsyncPowAdapter,
            $type,
            |$adapter| $inner
        );
    };
}

pub(crate) use export_async_pow_adapter;
pub(crate) use export_pow_adapter;
pub(crate) use impl_async_pow_adapter;
pub(crate) use impl_pow_adapter;
