use std::ops::Deref;
use std::sync::Arc;

use nostr_sdk::pool::monitor;
use uniffi::Object;

use crate::error::Result;
use crate::protocol::types::RelayUrl;
use crate::relay::RelayStatus;

// /// Monitor notification
// #[derive(Enum)]
// pub enum MonitorNotification {
//     /// Relay status changed
//     StatusChanged {
//         /// Relay URL
//         relay_url: Arc<RelayUrl>,
//         /// Status
//         status: RelayStatus,
//     },
// }
//
// impl From<monitor::MonitorNotification> for MonitorNotification {
//     fn from(notification: monitor::MonitorNotification) -> Self {
//         match notification {
//             monitor::MonitorNotification::StatusChanged { relay_url, status } => Self::StatusChanged {
//                 relay_url: Arc::new(relay_url.into()),
//                 status: status.into(),
//             },
//         }
//     }
// }

// #[derive(Object)]
// pub struct MonitorNotificationStream {
//     stream: Mutex<BoxedStream<monitor::MonitorNotification>>,
// }
//
// impl From<BoxedStream<monitor::MonitorNotification>> for MonitorNotificationStream {
//     fn from(stream: BoxedStream<monitor::MonitorNotification>) -> Self {
//         Self {
//             stream: Mutex::new(stream),
//         }
//     }
// }
//
// #[uniffi::export(async_runtime = "tokio")]
// impl MonitorNotificationStream {
//     pub async fn next(&self) -> Option<MonitorNotification> {
//         let mut stream = self.stream.lock().await;
//         let notification: monitor::MonitorNotification = stream.next().await?;
//         Some(notification.into())
//     }
// }

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait HandleMonitorNotification: Send + Sync {
    async fn relay_status_changed(&self, relay_url: Arc<RelayUrl>, status: RelayStatus);
}

#[derive(Object)]
pub struct Monitor {
    inner: monitor::Monitor,
}

impl Deref for Monitor {
    type Target = monitor::Monitor;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<monitor::Monitor> for Monitor {
    fn from(inner: monitor::Monitor) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl Monitor {
    /// Create a new monitor with the given channel size
    #[uniffi::constructor(default(channel_size = 4096))]
    pub fn new(channel_size: u64) -> Self {
        Self {
            inner: monitor::Monitor::new(channel_size as usize),
        }
    }

    // /// Subscribe to monitor notifications
    // ///
    // /// <div class="warning">When you call this method, you subscribe to the notifications channel from that precise moment. Anything received by relay/s before that moment is not included in the channel!</div>
    // pub fn subscribe(&self) -> MonitorNotificationStream {
    //     let receiver = self.inner.subscribe();
    //     let stream: BroadcastStream<monitor::MonitorNotification> = BroadcastStream::new(receiver);
    //     let boxed: BoxedStream<monitor::MonitorNotification> = Box::pin(stream);
    //     boxed.into()
    // }

    /// Handle notifications
    pub async fn handle_notifications(
        &self,
        handler: Arc<dyn HandleMonitorNotification>,
    ) -> Result<()> {
        let mut notifications = self.inner.subscribe();

        while let Ok(notification) = notifications.recv().await {
            match notification {
                monitor::MonitorNotification::StatusChanged { relay_url, status } => {
                    handler
                        .relay_status_changed(Arc::new(relay_url.into()), status.into())
                        .await;
                }
            }
        }

        Ok(())
    }
}
