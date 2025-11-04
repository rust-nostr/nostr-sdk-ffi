// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

use nostr::nips::nip46;
use nostr::{JsonUtil, Url};
use uniffi::{Enum, Object};

use crate::error::{NostrSdkError, Result};
use crate::protocol::event::UnsignedEvent;
use crate::protocol::key::PublicKey;

/// Nostr Connect Request (NIP46)
#[derive(Enum)]
pub enum NostrConnectRequest {
    /// Connect
    Connect {
        /// Remote public key
        remote_signer_public_key: Arc<PublicKey>,
        /// Optional secret
        secret: Option<String>,
    },
    /// Get public key
    GetPublicKey,
    /// Sign [`UnsignedEvent`]
    SignEvent { unsigned_event: Arc<UnsignedEvent> },
    /// Encrypt text (NIP04)
    Nip04Encrypt {
        /// Pubkey
        public_key: Arc<PublicKey>,
        /// Plain text
        text: String,
    },
    /// Decrypt (NIP04)
    Nip04Decrypt {
        /// Pubkey
        public_key: Arc<PublicKey>,
        /// Ciphertext
        ciphertext: String,
    },
    /// Encrypt text (NIP44)
    Nip44Encrypt {
        /// Pubkey
        public_key: Arc<PublicKey>,
        /// Plain text
        text: String,
    },
    /// Decrypt (NIP44)
    Nip44Decrypt {
        /// Pubkey
        public_key: Arc<PublicKey>,
        /// Ciphertext
        ciphertext: String,
    },
    /// Ping
    Ping,
}

impl From<nip46::NostrConnectRequest> for NostrConnectRequest {
    fn from(req: nip46::NostrConnectRequest) -> Self {
        match req {
            nip46::NostrConnectRequest::Connect {
                remote_signer_public_key,
                secret,
            } => Self::Connect {
                remote_signer_public_key: Arc::new(remote_signer_public_key.into()),
                secret,
            },
            nip46::NostrConnectRequest::GetPublicKey => Self::GetPublicKey,
            nip46::NostrConnectRequest::SignEvent(unsigned) => Self::SignEvent {
                unsigned_event: Arc::new(unsigned.into()),
            },
            nip46::NostrConnectRequest::Nip04Encrypt { public_key, text } => Self::Nip04Encrypt {
                public_key: Arc::new(public_key.into()),
                text,
            },
            nip46::NostrConnectRequest::Nip04Decrypt {
                public_key,
                ciphertext,
            } => Self::Nip04Decrypt {
                public_key: Arc::new(public_key.into()),
                ciphertext,
            },
            nip46::NostrConnectRequest::Nip44Encrypt { public_key, text } => Self::Nip44Encrypt {
                public_key: Arc::new(public_key.into()),
                text,
            },
            nip46::NostrConnectRequest::Nip44Decrypt {
                public_key,
                ciphertext,
            } => Self::Nip44Decrypt {
                public_key: Arc::new(public_key.into()),
                ciphertext,
            },
            nip46::NostrConnectRequest::Ping => Self::Ping,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct NostrConnectMetadata {
    inner: nip46::NostrConnectMetadata,
}

impl Deref for NostrConnectMetadata {
    type Target = nip46::NostrConnectMetadata;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[uniffi::export]
impl NostrConnectMetadata {
    /// New Nostr Connect Metadata
    #[uniffi::constructor]
    pub fn new(name: String) -> Self {
        Self {
            inner: nip46::NostrConnectMetadata::new(name),
        }
    }

    /// URL of the website requesting the connection
    pub fn url(&self, url: &str) -> Result<Self> {
        let url: Url = Url::parse(url)?;
        let mut builder = self.clone();
        builder.inner = builder.inner.url(url);
        Ok(builder)
    }

    /// Description of the `App`
    pub fn description(&self, description: String) -> Self {
        let mut builder = self.clone();
        builder.inner = builder.inner.description(description);
        builder
    }

    /// List of URLs for icons of the `App`
    pub fn icons(&self, icons: Vec<String>) -> Self {
        let icons: Vec<Url> = icons
            .into_iter()
            .filter_map(|u| Url::parse(&u).ok())
            .collect();
        let mut builder = self.clone();
        builder.inner = builder.inner.icons(icons);
        builder
    }

    /// Serialize as JSON string
    pub fn as_json(&self) -> Result<String> {
        Ok(self.inner.try_as_json()?)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Display, Eq, Hash)]
pub struct NostrConnectURI {
    inner: nip46::NostrConnectURI,
}

impl fmt::Display for NostrConnectURI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl From<nip46::NostrConnectURI> for NostrConnectURI {
    fn from(inner: nip46::NostrConnectURI) -> Self {
        Self { inner }
    }
}

impl Deref for NostrConnectURI {
    type Target = nip46::NostrConnectURI;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[uniffi::export]
impl NostrConnectURI {
    #[uniffi::constructor]
    pub fn parse(uri: &str) -> Result<Self> {
        Ok(Self {
            inner: nip46::NostrConnectURI::parse(uri)?,
        })
    }
}

#[derive(Enum)]
pub enum NostrConnectMessage {
    Request {
        id: String,
        method: String,
        params: Vec<String>,
    },
    Response {
        id: String,
        result: Option<String>,
        error: Option<String>,
    },
}

impl TryFrom<NostrConnectMessage> for nip46::NostrConnectMessage {
    type Error = NostrSdkError;

    fn try_from(value: NostrConnectMessage) -> Result<Self, Self::Error> {
        match value {
            NostrConnectMessage::Request { id, method, params } => Ok(Self::Request {
                id,
                method: nip46::NostrConnectMethod::from_str(&method)?,
                params,
            }),
            NostrConnectMessage::Response { id, result, error } => {
                Ok(Self::Response { id, result, error })
            }
        }
    }
}
