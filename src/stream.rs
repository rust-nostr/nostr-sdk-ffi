// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::sync::Arc;

use futures_util::StreamExt;
use nostr_sdk::pool::stream::ReceiverStream;
use tokio::sync::Mutex;
use uniffi::Object;

use crate::protocol::event::Event;

#[derive(Object)]
pub struct EventStream {
    stream: Mutex<ReceiverStream<nostr::Event>>,
}

impl From<ReceiverStream<nostr::Event>> for EventStream {
    fn from(stream: ReceiverStream<nostr::Event>) -> Self {
        Self {
            stream: Mutex::new(stream),
        }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl EventStream {
    pub async fn next(&self) -> Option<Arc<Event>> {
        let mut stream = self.stream.lock().await;
        let event: nostr::Event = stream.next().await?;
        Some(Arc::new(event.into()))
    }
}
