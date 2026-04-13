// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::pin::Pin;
use std::sync::Arc;

use futures_util::{Stream, StreamExt};
use nostr_sdk::{client, relay};
use tokio::sync::Mutex;
use uniffi::{Object, Record};

use super::notification::ClientNotification;
use crate::protocol::event::Event;
use crate::protocol::types::RelayUrl;

#[derive(Object)]
pub struct ClientNotificationStream {
    stream: Mutex<Pin<Box<dyn Stream<Item = client::ClientNotification> + Send>>>,
}

impl From<Pin<Box<dyn Stream<Item = client::ClientNotification> + Send>>>
    for ClientNotificationStream
{
    fn from(stream: Pin<Box<dyn Stream<Item = client::ClientNotification> + Send>>) -> Self {
        Self {
            stream: Mutex::new(stream),
        }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl ClientNotificationStream {
    /// Consumes the next item from the stream.
    ///
    /// Returns null if the stream is terminated.
    pub async fn next(&self) -> Option<ClientNotification> {
        let mut stream = self.stream.lock().await;
        let item: client::ClientNotification = stream.next().await?;
        Some(item.into())
    }
}

#[derive(Record)]
pub struct ClientEventStreamItem {
    pub relay_url: Arc<RelayUrl>,
    pub event: Option<Arc<Event>>,
    pub error: Option<String>,
}

impl From<(nostr::RelayUrl, Result<nostr::Event, relay::Error>)> for ClientEventStreamItem {
    fn from((url, result): (nostr::RelayUrl, Result<nostr::Event, relay::Error>)) -> Self {
        let (event, error) = match result {
            Ok(event) => (Some(Arc::new(event.into())), None),
            Err(err) => (None, Some(err.to_string())),
        };

        Self {
            relay_url: Arc::new(url.into()),
            event,
            error,
        }
    }
}

#[derive(Object)]
#[allow(clippy::type_complexity)]
pub struct ClientEventStream {
    stream: Mutex<
        Pin<Box<dyn Stream<Item = (nostr::RelayUrl, Result<nostr::Event, relay::Error>)> + Send>>,
    >,
}

impl From<Pin<Box<dyn Stream<Item = (nostr::RelayUrl, Result<nostr::Event, relay::Error>)> + Send>>>
    for ClientEventStream
{
    fn from(
        stream: Pin<
            Box<dyn Stream<Item = (nostr::RelayUrl, Result<nostr::Event, relay::Error>)> + Send>,
        >,
    ) -> Self {
        Self {
            stream: Mutex::new(stream),
        }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl ClientEventStream {
    /// Consumes the next item from the stream.
    ///
    /// Returns null if the stream is terminated.
    pub async fn next(&self) -> Option<ClientEventStreamItem> {
        let mut stream = self.stream.lock().await;
        let item: (nostr::RelayUrl, Result<nostr::Event, relay::Error>) = stream.next().await?;
        Some(item.into())
    }
}
