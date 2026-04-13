// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use nostr::SubscriptionId;
use nostr_sdk::client;
use uniffi::Record;

use crate::protocol::event::EventId;
use crate::protocol::types::RelayUrl;

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

impl From<client::Output<()>> for Output {
    #[inline]
    fn from(output: client::Output<()>) -> Self {
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

impl From<client::Output<nostr::EventId>> for SendEventOutput {
    fn from(output: client::Output<nostr::EventId>) -> Self {
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

impl From<client::Output<SubscriptionId>> for SubscribeOutput {
    fn from(output: client::Output<SubscriptionId>) -> Self {
        let out = convert_output(output.success, output.failed);
        Self {
            id: output.val.to_string(),
            success: out.success,
            failed: out.failed,
        }
    }
}

#[derive(Record)]
pub struct ClientSyncSummarySendFailureItem {
    pub id: Arc<EventId>,
    pub error: String,
}

/// Client sync summary
#[derive(Record)]
pub struct ClientSyncSummary {
    /// The IDs that were stored locally
    pub local: Vec<Arc<EventId>>,
    /// The IDs that were missing locally (stored on relay)
    pub remote: HashMap<Arc<EventId>, Vec<Arc<RelayUrl>>>,
    /// Events that are **successfully** sent to relays during reconciliation
    pub sent: HashMap<Arc<EventId>, Vec<Arc<RelayUrl>>>,
    /// Event that are **successfully** received from relay
    pub received: HashMap<Arc<EventId>, Vec<Arc<RelayUrl>>>,
    /// Events that failed to send to relays during reconciliation
    pub send_failures: HashMap<Arc<RelayUrl>, Vec<ClientSyncSummarySendFailureItem>>,
}

impl From<client::SyncSummary> for ClientSyncSummary {
    fn from(value: client::SyncSummary) -> Self {
        Self {
            local: value
                .local
                .into_iter()
                .map(|e| Arc::new(e.into()))
                .collect(),
            remote: value
                .remote
                .into_iter()
                .map(|(e, r)| {
                    (
                        Arc::new(e.into()),
                        r.into_iter().map(|u| Arc::new(u.into())).collect(),
                    )
                })
                .collect(),
            sent: value
                .sent
                .into_iter()
                .map(|(e, r)| {
                    (
                        Arc::new(e.into()),
                        r.into_iter().map(|u| Arc::new(u.into())).collect(),
                    )
                })
                .collect(),
            received: value
                .received
                .into_iter()
                .map(|(e, r)| {
                    (
                        Arc::new(e.into()),
                        r.into_iter().map(|u| Arc::new(u.into())).collect(),
                    )
                })
                .collect(),
            send_failures: value
                .send_failures
                .into_iter()
                .map(|(url, map)| {
                    (
                        Arc::new(url.into()),
                        map.into_iter()
                            .map(|(id, e)| ClientSyncSummarySendFailureItem {
                                id: Arc::new(id.into()),
                                error: e,
                            })
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}

/// Client sync summary output
#[derive(Record)]
pub struct ClientSyncSummaryOutput {
    /// Reconciliation report
    pub report: ClientSyncSummary,
    /// Set of relays that success
    pub success: Vec<Arc<RelayUrl>>,
    /// Map of relays that failed, with related errors.
    pub failed: HashMap<Arc<RelayUrl>, String>,
}

impl From<client::Output<client::SyncSummary>> for ClientSyncSummaryOutput {
    fn from(output: client::Output<client::SyncSummary>) -> Self {
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
