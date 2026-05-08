// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::sync::Arc;

use nostr::JsonUtil;
use nostr::nips::nip11;
use uniffi::{Object, Record};

use crate::error::Result;
use crate::protocol::types::Timestamp;

#[derive(Debug, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct RelayInformationDocument {
    inner: nip11::RelayInformationDocument,
}

impl From<nip11::RelayInformationDocument> for RelayInformationDocument {
    fn from(inner: nip11::RelayInformationDocument) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl RelayInformationDocument {
    /// Parse NIP-11 relay information document from JSON
    #[uniffi::constructor]
    pub fn from_json(json: &str) -> Result<Self> {
        Ok(Self {
            inner: nip11::RelayInformationDocument::from_json(json)?,
        })
    }

    /// Serialize as JSON
    pub fn as_json(&self) -> Result<String> {
        Ok(self.inner.try_as_json()?)
    }

    pub fn name(&self) -> Option<String> {
        self.inner.name.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.inner.description.clone()
    }

    pub fn pubkey(&self) -> Option<String> {
        self.inner.pubkey.clone()
    }

    pub fn contact(&self) -> Option<String> {
        self.inner.contact.clone()
    }

    pub fn supported_nips(&self) -> Option<Vec<u16>> {
        self.inner.supported_nips.clone()
    }

    pub fn software(&self) -> Option<String> {
        self.inner.software.clone()
    }

    pub fn version(&self) -> Option<String> {
        self.inner.version.clone()
    }

    pub fn limitation(&self) -> Option<Limitation> {
        self.inner.limitation.clone().map(|l| l.into())
    }

    pub fn payments_url(&self) -> Option<String> {
        self.inner.payments_url.clone()
    }

    pub fn fees(&self) -> Option<FeeSchedules> {
        self.inner.fees.clone().map(|f| f.into())
    }

    pub fn icon(&self) -> Option<String> {
        self.inner.icon.clone()
    }

    pub fn banner(&self) -> Option<String> {
        self.inner.banner.clone()
    }

    pub fn self_pubkey(&self) -> Option<String> {
        self.inner.self_pubkey.clone()
    }

    pub fn terms_of_service(&self) -> Option<String> {
        self.inner.terms_of_service.clone()
    }
}

/// These are limitations imposed by the relay on clients. Your client should
/// expect that requests which exceed these practical limitations are rejected or fail immediately.
#[derive(Record)]
pub struct Limitation {
    /// Maximum number of bytes for incoming JSON that the relay will attempt to decode and act upon
    pub max_message_length: Option<i32>,
    /// Total number of subscriptions that may be active on a single websocket connection
    pub max_subscriptions: Option<i32>,
    /// Relay will clamp each filter's limit value to this number
    pub max_limit: Option<i32>,
    /// Maximum length of subscription id as a string
    pub max_subid_length: Option<i32>,
    /// Maximum number of elements in the tags list
    pub max_event_tags: Option<i32>,
    /// Maximum number of characters in the content field of any event
    pub max_content_length: Option<i32>,
    /// New events will require at least this difficulty of PoW
    pub min_pow_difficulty: Option<i32>,
    /// Relay requires NIP42 authentication to happen before a new connection may perform any other action
    pub auth_required: Option<bool>,
    /// Relay requires payment before a new connection may perform any action
    pub payment_required: Option<bool>,
    /// Relay requires some kind of condition to be fulfilled to accept events
    pub restricted_writes: Option<bool>,
    /// 'created_at' lower limit
    pub created_at_lower_limit: Option<Arc<Timestamp>>,
    /// 'created_at' upper limit
    pub created_at_upper_limit: Option<Arc<Timestamp>>,
    /// Maximum returned events if you send a filter without a `limit`
    pub default_limit: Option<i32>,
}

impl From<nip11::Limitation> for Limitation {
    fn from(inner: nip11::Limitation) -> Self {
        let nip11::Limitation {
            max_message_length,
            max_subscriptions,
            max_limit,
            max_subid_length,
            max_event_tags,
            max_content_length,
            min_pow_difficulty,
            auth_required,
            payment_required,
            restricted_writes,
            created_at_lower_limit,
            created_at_upper_limit,
            default_limit,
        } = inner;
        Self {
            max_message_length,
            max_subscriptions,
            max_limit,
            max_subid_length,
            max_event_tags,
            max_content_length,
            min_pow_difficulty,
            auth_required,
            payment_required,
            restricted_writes,
            created_at_lower_limit: created_at_lower_limit.map(|c| Arc::new(c.into())),
            created_at_upper_limit: created_at_upper_limit.map(|c| Arc::new(c.into())),
            default_limit,
        }
    }
}

/// Available fee schedules
#[derive(Record)]
pub struct FeeSchedules {
    /// Fees for admission to use the relay
    pub admission: Vec<FeeSchedule>,
    /// Fees for subscription to use the relay
    pub subscription: Vec<FeeSchedule>,
    /// Fees to publish to the relay
    pub publication: Vec<FeeSchedule>,
}

impl From<nip11::FeeSchedules> for FeeSchedules {
    fn from(inner: nip11::FeeSchedules) -> Self {
        let nip11::FeeSchedules {
            admission,
            subscription,
            publication,
        } = inner;
        Self {
            admission: admission.into_iter().map(|a| a.into()).collect(),
            subscription: subscription.into_iter().map(|s| s.into()).collect(),
            publication: publication.into_iter().map(|p| p.into()).collect(),
        }
    }
}

/// The specific information about a fee schedule
#[derive(Record)]
pub struct FeeSchedule {
    /// The fee amount
    pub amount: i32,
    /// The denomination of the feed
    pub unit: String,
    /// The duration for which the fee is valid
    pub period: Option<i32>,
    /// The event kinds the fee allows the client to publish to the relay
    pub kinds: Option<Vec<u16>>,
}

impl From<nip11::FeeSchedule> for FeeSchedule {
    fn from(inner: nip11::FeeSchedule) -> Self {
        let nip11::FeeSchedule {
            amount,
            unit,
            period,
            kinds,
        } = inner;
        Self {
            amount,
            unit,
            period,
            kinds,
        }
    }
}
