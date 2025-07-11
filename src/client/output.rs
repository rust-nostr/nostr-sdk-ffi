// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use nostr_sdk::{SubscriptionId, pool};
use uniffi::Record;

use crate::protocol::event::EventId;
use crate::protocol::types::RelayUrl;
use crate::relay::Reconciliation;

/// Output
///
/// Send or negentropy reconciliation output
#[derive(Record)]
pub struct Output {
    /// Set of relays that success
    pub success: Vec<Arc<RelayUrl>>,
    /// Map of relays that failed, with related errors.
    pub failed: HashMap<Arc<RelayUrl>, String>,
}

impl From<pool::Output<()>> for Output {
    #[inline]
    fn from(output: pool::Output<()>) -> Self {
        convert_output(output.success, output.failed)
    }
}

/// Send event output
#[derive(Record)]
pub struct SendEventOutput {
    /// Event ID
    pub id: Arc<EventId>,
    /// Set of relays that success
    pub success: Vec<Arc<RelayUrl>>,
    /// Map of relays that failed, with related errors.
    pub failed: HashMap<Arc<RelayUrl>, String>,
}

impl From<pool::Output<nostr_sdk::EventId>> for SendEventOutput {
    fn from(output: pool::Output<nostr_sdk::EventId>) -> Self {
        let out = convert_output(output.success, output.failed);
        Self {
            id: Arc::new(output.val.into()),
            success: out.success,
            failed: out.failed,
        }
    }
}

/// Subscribe output
#[derive(Record)]
pub struct SubscribeOutput {
    /// Subscription ID
    pub id: String,
    /// Set of relays that success
    pub success: Vec<Arc<RelayUrl>>,
    /// Map of relays that failed, with related errors.
    pub failed: HashMap<Arc<RelayUrl>, String>,
}

impl From<pool::Output<SubscriptionId>> for SubscribeOutput {
    fn from(output: pool::Output<SubscriptionId>) -> Self {
        let out = convert_output(output.success, output.failed);
        Self {
            id: output.val.to_string(),
            success: out.success,
            failed: out.failed,
        }
    }
}

/// Reconciliation output
#[derive(Record)]
pub struct ReconciliationOutput {
    /// Reconciliation report
    pub report: Reconciliation,
    /// Set of relays that success
    pub success: Vec<Arc<RelayUrl>>,
    /// Map of relays that failed, with related errors.
    pub failed: HashMap<Arc<RelayUrl>, String>,
}

impl From<pool::Output<pool::Reconciliation>> for ReconciliationOutput {
    fn from(output: pool::Output<pool::Reconciliation>) -> Self {
        let out = convert_output(output.success, output.failed);
        Self {
            report: output.val.into(),
            success: out.success,
            failed: out.failed,
        }
    }
}

fn convert_output(
    success: HashSet<nostr::RelayUrl>,
    failed: HashMap<nostr::RelayUrl, String>,
) -> Output {
    Output {
        success: success.into_iter().map(|u| Arc::new(u.into())).collect(),
        failed: failed
            .into_iter()
            .map(|(u, e)| (Arc::new(u.into()), e))
            .collect(),
    }
}
