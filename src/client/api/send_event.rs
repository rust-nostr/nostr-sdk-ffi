use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use nostr_sdk::client;
use uniffi::Object;

use crate::client::Client;
use crate::client::output::SendEventOutput;
use crate::error::Result;
use crate::protocol::event::Event;
use crate::protocol::types::RelayUrl;

enum InnerSendEventTarget {
    // All WRITE relays
    Broadcast,
    // To specific relays
    To(Vec<nostr::RelayUrl>),
    // To NIP-17 relays
    ToNip17,
    // To NIP-65 relays
    ToNip65,
}

/// Where to send the event
#[derive(Object)]
pub struct SendEventTarget(InnerSendEventTarget);

#[uniffi::export]
impl SendEventTarget {
    /// Send event to all relays with `WRITE` capability.
    #[uniffi::constructor]
    pub fn broadcast() -> Self {
        Self(InnerSendEventTarget::Broadcast)
    }

    /// Send event to specific relays
    #[uniffi::constructor]
    pub fn to(relays: Vec<Arc<RelayUrl>>) -> Self {
        Self(InnerSendEventTarget::To(
            relays
                .into_iter()
                .map(|r| r.as_ref().deref().clone())
                .collect(),
        ))
    }

    /// Send event to NIP-17 relays
    #[uniffi::constructor]
    pub fn to_nip17() -> Self {
        Self(InnerSendEventTarget::ToNip17)
    }

    /// Send event to NIP-65 relays
    #[uniffi::constructor]
    pub fn to_nip65() -> Self {
        Self(InnerSendEventTarget::ToNip65)
    }
}

/// Policy for relay `OK` acknowledgements when sending events.
///
/// This policy controls whether each relay send waits for an `OK` response
/// after dispatching the `EVENT` message.
#[derive(Object)]
pub struct AckPolicy {
    inner: client::AckPolicy,
}

impl AckPolicy {
    /// Wait for relay `OK` acknowledgement from each selected relay (default).
    #[inline]
    pub const fn all() -> Self {
        Self {
            inner: client::AckPolicy::all(),
        }
    }

    /// Do not wait for relay `OK` acknowledgements.
    ///
    /// The operation still sends to all selected relays, but each relay result
    /// is reported immediately after dispatch.
    #[inline]
    pub const fn none() -> Self {
        Self {
            inner: client::AckPolicy::none(),
        }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl Client {
    /// Send event
    #[uniffi::method(default(target = None, ack_policy = None, ok_timeout = None, authentication_timeout = None))]
    pub async fn send_event(
        &self,
        event: &Event,
        target: Option<Arc<SendEventTarget>>,
        ack_policy: Option<Arc<AckPolicy>>,
        ok_timeout: Option<Duration>,
        authentication_timeout: Option<Duration>,
    ) -> Result<SendEventOutput> {
        let mut builder = self.inner.send_event(event.deref());

        if let Some(target) = &target {
            match &target.0 {
                InnerSendEventTarget::Broadcast => {
                    builder = builder.broadcast();
                }
                InnerSendEventTarget::To(relays) => {
                    builder = builder.to(relays);
                }
                InnerSendEventTarget::ToNip17 => {
                    builder = builder.to_nip17();
                }
                InnerSendEventTarget::ToNip65 => {
                    builder = builder.to_nip65();
                }
            }
        }

        if let Some(ack_policy) = ack_policy {
            builder = builder.ack_policy(ack_policy.inner.clone());
        }

        if let Some(ok_timeout) = ok_timeout {
            builder = builder.ok_timeout(ok_timeout);
        }
        if let Some(authentication_timeout) = authentication_timeout {
            builder = builder.authentication_timeout(authentication_timeout);
        }

        Ok(builder.await?.into())
    }
}
