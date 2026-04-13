// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use nostr_sdk::relay;
use uniffi::Enum;

#[derive(Enum)]
pub enum RelayStatus {
    /// Initialized
    Initialized,
    /// Pending
    Pending,
    /// Connecting
    Connecting,
    /// Connected
    Connected,
    /// Disconnected, will retry to connect again
    Disconnected,
    /// Completely disconnected
    Terminated,
    /// The relay has been banned.
    Banned,
    /// Relay is sleeping
    Sleeping,
    /// Shutdown
    Shutdown,
}

impl From<relay::RelayStatus> for RelayStatus {
    fn from(value: relay::RelayStatus) -> Self {
        match value {
            relay::RelayStatus::Initialized => Self::Initialized,
            relay::RelayStatus::Pending => Self::Pending,
            relay::RelayStatus::Connecting => Self::Connecting,
            relay::RelayStatus::Connected => Self::Connected,
            relay::RelayStatus::Disconnected => Self::Disconnected,
            relay::RelayStatus::Terminated => Self::Terminated,
            relay::RelayStatus::Banned => Self::Banned,
            relay::RelayStatus::Sleeping => Self::Sleeping,
            relay::RelayStatus::Shutdown => Self::Shutdown,
        }
    }
}
