macro_rules! impl_nostr_signer {
    ($type:ty, |$signer:ident| $inner:expr) => {
        impl_nostr_signer!(
            @impl
            []
            $crate::protocol::signer::NostrSigner,
            $type,
            |$signer| $inner
        );
    };

    (@impl [$($attr:tt)*] $trait:path, $type:ty, |$signer:ident| $inner:expr) => {
        $($attr)*
        impl $trait for $type {
            fn get_public_key(
                &self,
            ) -> $crate::error::Result<::std::sync::Arc<$crate::protocol::key::PublicKey>> {
                let $signer = self;
                let signer = $inner;
                Ok(::std::sync::Arc::new(
                    ::nostr::key::GetPublicKey::get_public_key(signer)?.into(),
                ))
            }

            fn sign_event(
                &self,
                unsigned_event: ::std::sync::Arc<$crate::protocol::event::UnsignedEvent>,
            ) -> $crate::error::Result<::std::sync::Arc<$crate::protocol::event::Event>> {
                let $signer = self;
                let signer = $inner;
                Ok(::std::sync::Arc::new(
                    ::nostr::event::SignEvent::sign_event(signer, (**unsigned_event).clone())?
                        .into(),
                ))
            }

            fn nip04_encrypt(
                &self,
                public_key: ::std::sync::Arc<$crate::protocol::key::PublicKey>,
                content: String,
            ) -> $crate::error::Result<String> {
                let $signer = self;
                let signer = $inner;
                Ok(::nostr::nips::nip04::Nip04::nip04_encrypt(
                    signer,
                    &**public_key,
                    &content,
                )?)
            }

            fn nip04_decrypt(
                &self,
                public_key: ::std::sync::Arc<$crate::protocol::key::PublicKey>,
                encrypted_content: String,
            ) -> $crate::error::Result<String> {
                let $signer = self;
                let signer = $inner;
                Ok(::nostr::nips::nip04::Nip04::nip04_decrypt(
                    signer,
                    &**public_key,
                    &encrypted_content,
                )?)
            }

            fn nip44_encrypt(
                &self,
                public_key: ::std::sync::Arc<$crate::protocol::key::PublicKey>,
                content: String,
            ) -> $crate::error::Result<String> {
                let $signer = self;
                let signer = $inner;
                Ok(::nostr::nips::nip44::Nip44::nip44_encrypt(
                    signer,
                    &**public_key,
                    &content,
                )?)
            }

            fn nip44_decrypt(
                &self,
                public_key: ::std::sync::Arc<$crate::protocol::key::PublicKey>,
                payload: String,
            ) -> $crate::error::Result<String> {
                let $signer = self;
                let signer = $inner;
                Ok(::nostr::nips::nip44::Nip44::nip44_decrypt(
                    signer,
                    &**public_key,
                    &payload,
                )?)
            }
        }
    };
}

macro_rules! export_nostr_signer {
    ($type:ty, |$signer:ident| $inner:expr) => {
        $crate::protocol::signer::impl_nostr_signer!(
            @impl
            [#[uniffi::export]]
            NostrSigner,
            $type,
            |$signer| $inner
        );
    };
}

macro_rules! impl_async_nostr_signer {
    ($type:ty, |$signer:ident| $inner:expr) => {
        impl_async_nostr_signer!(
            @impl
            []
            $crate::protocol::signer::AsyncNostrSigner,
            $type,
            |$signer| $inner
        );
    };

    (@impl [$($attr:tt)*] $trait:path, $type:ty, |$signer:ident| $inner:expr) => {
        $($attr)*
        #[async_trait::async_trait]
        impl $trait for $type {
            async fn get_public_key_async(
                &self,
            ) -> $crate::error::Result<Option<::std::sync::Arc<$crate::protocol::key::PublicKey>>>
            {
                let $signer = self;
                let signer = $inner;
                Ok(Some(::std::sync::Arc::new(
                    $crate::future::assume_send(
                        ::nostr::key::AsyncGetPublicKey::get_public_key_async(signer),
                    )
                    .await?
                    .into(),
                )))
            }

            async fn sign_event_async(
                &self,
                unsigned_event: ::std::sync::Arc<$crate::protocol::event::UnsignedEvent>,
            ) -> $crate::error::Result<Option<::std::sync::Arc<$crate::protocol::event::Event>>>
            {
                let $signer = self;
                let signer = $inner;
                Ok(Some(::std::sync::Arc::new(
                    $crate::future::assume_send(
                        ::nostr::event::AsyncSignEvent::sign_event_async(
                            signer,
                            (**unsigned_event).clone(),
                        ),
                    )
                    .await?
                    .into(),
                )))
            }

            async fn nip04_encrypt_async(
                &self,
                public_key: ::std::sync::Arc<$crate::protocol::key::PublicKey>,
                content: String,
            ) -> $crate::error::Result<String> {
                let $signer = self;
                let signer = $inner;
                Ok($crate::future::assume_send(
                    <_ as ::nostr::nips::nip04::AsyncNip04>::nip04_encrypt_async(
                        signer,
                        &**public_key,
                        &content,
                    ),
                )
                .await?)
            }

            async fn nip04_decrypt_async(
                &self,
                public_key: ::std::sync::Arc<$crate::protocol::key::PublicKey>,
                encrypted_content: String,
            ) -> $crate::error::Result<String> {
                let $signer = self;
                let signer = $inner;
                Ok($crate::future::assume_send(
                    <_ as ::nostr::nips::nip04::AsyncNip04>::nip04_decrypt_async(
                        signer,
                        &**public_key,
                        &encrypted_content,
                    ),
                )
                .await?)
            }

            async fn nip44_encrypt_async(
                &self,
                public_key: ::std::sync::Arc<$crate::protocol::key::PublicKey>,
                content: String,
            ) -> $crate::error::Result<String> {
                let $signer = self;
                let signer = $inner;
                Ok($crate::future::assume_send(
                    <_ as ::nostr::nips::nip44::AsyncNip44>::nip44_encrypt_async(
                        signer,
                        &**public_key,
                        &content,
                    ),
                )
                .await?)
            }

            async fn nip44_decrypt_async(
                &self,
                public_key: ::std::sync::Arc<$crate::protocol::key::PublicKey>,
                payload: String,
            ) -> $crate::error::Result<String> {
                let $signer = self;
                let signer = $inner;
                Ok($crate::future::assume_send(
                    <_ as ::nostr::nips::nip44::AsyncNip44>::nip44_decrypt_async(
                        signer,
                        &**public_key,
                        &payload,
                    ),
                )
                .await?)
            }
        }
    };
}

macro_rules! export_async_nostr_signer {
    ($type:ty, |$signer:ident| $inner:expr) => {
        $crate::protocol::signer::impl_async_nostr_signer!(
            @impl
            [#[uniffi::export(async_runtime = "tokio")]]
            AsyncNostrSigner,
            $type,
            |$signer| $inner
        );
    };
}

pub(crate) use export_async_nostr_signer;
pub(crate) use export_nostr_signer;
pub(crate) use impl_async_nostr_signer;
pub(crate) use impl_nostr_signer;
