// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

use nostr::JsonUtil;
use nostr::nips::nip01;
use uniffi::{Object, Record};

use crate::error::Result;
use crate::protocol::event::Kind;
use crate::protocol::key::PublicKey;
use crate::protocol::util::JsonValue;

/// Coordinate for event (`a` tag)
#[derive(Debug, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Display, Eq, Hash)]
pub struct Coordinate {
    inner: nip01::Coordinate,
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Deref for Coordinate {
    type Target = nip01::Coordinate;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<nip01::Coordinate> for Coordinate {
    fn from(inner: nip01::Coordinate) -> Self {
        Self { inner }
    }
}

impl From<Coordinate> for nip01::Coordinate {
    fn from(value: Coordinate) -> Self {
        Self {
            kind: value.inner.kind,
            public_key: value.inner.public_key,
            identifier: value.inner.identifier,
        }
    }
}

#[uniffi::export]
impl Coordinate {
    #[uniffi::constructor(default(identifier = ""))]
    pub fn new(kind: &Kind, public_key: &PublicKey, identifier: String) -> Self {
        Self {
            inner: nip01::Coordinate {
                kind: **kind,
                public_key: **public_key,
                identifier,
            },
        }
    }

    #[uniffi::constructor]
    pub fn parse(coordinate: &str) -> Result<Self> {
        Ok(nip01::Coordinate::from_str(coordinate)?.into())
    }

    pub fn kind(&self) -> Kind {
        self.inner.kind.into()
    }

    pub fn public_key(&self) -> PublicKey {
        self.inner.public_key.into()
    }

    pub fn identifier(&self) -> String {
        self.inner.identifier.clone()
    }

    /// Check if the coordinate is valid.
    ///
    /// Returns `false` if:
    /// - the `Kind` is `replaceable` and the identifier is not empty
    /// - the `Kind` is `addressable` and the identifier is empty
    pub fn verify(&self) -> bool {
        self.inner.verify().is_ok()
    }
}

#[derive(Record)]
pub struct MetadataRecord {
    /// Name
    #[uniffi(default = None)]
    pub name: Option<String>,
    /// Display name
    #[uniffi(default = None)]
    pub display_name: Option<String>,
    /// Description
    #[uniffi(default = None)]
    pub about: Option<String>,
    /// Website url
    #[uniffi(default = None)]
    pub website: Option<String>,
    /// Picture url
    #[uniffi(default = None)]
    pub picture: Option<String>,
    /// Banner url
    #[uniffi(default = None)]
    pub banner: Option<String>,
    /// NIP05 (ex. name@example.com)
    #[uniffi(default = None)]
    pub nip05: Option<String>,
    /// LNURL
    #[uniffi(default = None)]
    pub lud06: Option<String>,
    /// Lightning Address
    #[uniffi(default = None)]
    pub lud16: Option<String>,
    /// Additional custom metadata
    #[uniffi(default = None)]
    pub custom: Option<HashMap<String, JsonValue>>,
}

impl From<MetadataRecord> for nostr::Metadata {
    fn from(value: MetadataRecord) -> Self {
        Self {
            name: value.name,
            display_name: value.display_name,
            about: value.about,
            website: value.website,
            picture: value.picture,
            banner: value.banner,
            nip05: value.nip05,
            lud06: value.lud06,
            lud16: value.lud16,
            ..Default::default()
        }
    }
}

impl From<nostr::Metadata> for MetadataRecord {
    fn from(value: nostr::Metadata) -> Self {
        Self {
            name: value.name,
            display_name: value.display_name,
            about: value.about,
            website: value.website,
            picture: value.picture,
            banner: value.banner,
            nip05: value.nip05,
            lud06: value.lud06,
            lud16: value.lud16,
            custom: Some(
                value
                    .custom
                    .into_iter()
                    .filter_map(|(k, v)| Some((k, v.try_into().ok()?)))
                    .collect(),
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct Metadata {
    inner: nip01::Metadata,
}

impl Deref for Metadata {
    type Target = nip01::Metadata;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<nip01::Metadata> for Metadata {
    fn from(inner: nip01::Metadata) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl Metadata {
    #[uniffi::constructor]
    pub fn from_record(r: MetadataRecord) -> Self {
        Self { inner: r.into() }
    }

    /// Parse metadata from JSON
    #[uniffi::constructor]
    pub fn from_json(json: String) -> Result<Self> {
        Ok(Self {
            inner: nip01::Metadata::from_json(json)?,
        })
    }

    pub fn as_record(&self) -> MetadataRecord {
        self.inner.clone().into()
    }

    pub fn as_json(&self) -> Result<String> {
        Ok(self.inner.try_as_json()?)
    }

    pub fn as_pretty_json(&self) -> Result<String> {
        Ok(self.inner.try_as_pretty_json()?)
    }
}
