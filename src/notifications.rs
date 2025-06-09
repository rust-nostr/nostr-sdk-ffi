// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::sync::Arc;

use crate::protocol::event::Event;
use crate::protocol::message::RelayMessage;
use crate::protocol::types::RelayUrl;

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait HandleNotification: Send + Sync {
    async fn handle_msg(&self, relay_url: Arc<RelayUrl>, msg: Arc<RelayMessage>);
    async fn handle(&self, relay_url: Arc<RelayUrl>, subscription_id: String, event: Arc<Event>);
}
