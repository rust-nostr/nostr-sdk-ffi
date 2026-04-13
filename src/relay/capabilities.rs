use std::ops::Deref;

use nostr_sdk::relay;
use uniffi::Object;

/// Relay capabilities
///
/// Represents what operations a relay can perform.
/// Multiple capabilities can be combined using bitwise OR operations.
#[derive(Debug, PartialEq, Eq, Hash, Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct RelayCapabilities(relay::RelayCapabilities);

impl Deref for RelayCapabilities {
    type Target = relay::RelayCapabilities;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<relay::RelayCapabilities> for RelayCapabilities {
    #[inline]
    fn from(caps: relay::RelayCapabilities) -> Self {
        Self(caps)
    }
}

#[uniffi::export]
impl RelayCapabilities {
    /// No capabilities
    #[inline]
    #[uniffi::constructor]
    pub fn none() -> Self {
        relay::RelayCapabilities::NONE.into()
    }

    /// Can perform read operations (i.e., fetch/query events)
    #[inline]
    #[uniffi::constructor]
    pub fn read() -> Self {
        relay::RelayCapabilities::READ.into()
    }

    /// Can perform write operations (i.e., publish events)
    #[inline]
    #[uniffi::constructor]
    pub fn write() -> Self {
        relay::RelayCapabilities::WRITE.into()
    }

    /// Gossip relay for NIP-17/NIP-65 (implies read/write)
    #[inline]
    #[uniffi::constructor]
    pub fn gossip() -> Self {
        relay::RelayCapabilities::GOSSIP.into()
    }

    /// Discovery relay for relay lists (i.e., kind 10002)
    #[inline]
    #[uniffi::constructor]
    pub fn discovery() -> Self {
        relay::RelayCapabilities::DISCOVERY.into()
    }

    /// Create new capabilities from raw bits
    #[inline]
    #[uniffi::constructor]
    pub fn from_bits(bits: u64) -> Self {
        relay::RelayCapabilities::from_bits(bits).into()
    }

    /// Get raw bits value
    #[inline]
    pub fn bits(&self) -> u64 {
        self.0.bits()
    }
}
