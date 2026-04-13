use std::sync::Arc;

use nostr_sdk::client;
use uniffi::Enum;

use crate::protocol::event::Event;
use crate::protocol::message::RelayMessage;
use crate::protocol::types::RelayUrl;

/// Nostr client notification
#[derive(Enum)]
pub enum ClientNotification {
    /// Received a new [`Event`] from a relay.
    ///
    /// This notification is sent only the **first time** the [`Event`] is seen.
    /// Events sent by this client are not included.
    /// This is useful when you only need to process new incoming events
    /// and avoid handling the same events multiple times.
    ///
    /// If you require notifications for all messages, including previously sent or received events,
    /// consider using the [`ClientNotification::Message`] variant instead.
    NewEvent {
        /// The URL of the relay from which the event was received.
        relay_url: Arc<RelayUrl>,
        /// Subscription ID
        subscription_id: String,
        /// The received event.
        event: Arc<Event>,
    },
    /// Received a [`RelayMessage`].
    ///
    /// This notification is sent **every time** a [`RelayMessage`] is received,
    /// regardless of whether it has been received before.
    ///
    /// May includes messages wrapping events that were sent by this client.
    Message {
        /// The URL of the relay from which the message was received.
        relay_url: Arc<RelayUrl>,
        /// The received relay message.
        message: Arc<RelayMessage>,
    },
    /// Shutdown
    ///
    /// This notification variant is sent after [`Client::shutdown`] method is called and all connections have been closed.
    Shutdown,
}

impl From<client::ClientNotification> for ClientNotification {
    fn from(notification: client::ClientNotification) -> Self {
        match notification {
            client::ClientNotification::Event {
                relay_url,
                subscription_id,
                event,
            } => Self::NewEvent {
                relay_url: Arc::new(relay_url.into()),
                subscription_id: subscription_id.to_string(),
                event: Arc::new((*event).into()),
            },
            client::ClientNotification::Message { relay_url, message } => Self::Message {
                relay_url: Arc::new(relay_url.into()),
                message: Arc::new((*message).into()),
            },
            client::ClientNotification::Shutdown => Self::Shutdown,
        }
    }
}
