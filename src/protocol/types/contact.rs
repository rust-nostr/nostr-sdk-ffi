// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;
use std::sync::Arc;

use nostr::nips::nip02;
use uniffi::Record;

use crate::protocol::key::PublicKey;
use crate::protocol::types::RelayUrl;

#[derive(Record)]
pub struct Contact {
    pub public_key: Arc<PublicKey>,
    #[uniffi(default = None)]
    pub relay_url: Option<Arc<RelayUrl>>,
    #[uniffi(default = None)]
    pub alias: Option<String>,
}

impl From<Contact> for nip02::Contact {
    fn from(contact: Contact) -> Self {
        nip02::Contact {
            public_key: **contact.public_key,
            relay_url: contact.relay_url.map(|u| u.as_ref().deref().clone()),
            alias: contact.alias,
        }
    }
}
