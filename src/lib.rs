// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

#![allow(clippy::new_without_default)]
#![allow(clippy::len_without_is_empty)]

pub mod client;
#[cfg(feature = "connect")]
pub mod connect;
pub mod database;
pub mod error;
#[cfg(feature = "logger")]
pub mod logger;
pub mod negentropy;
pub mod notifications;
#[cfg(feature = "nwc")]
pub mod nwc;
pub mod parser;
pub mod policy;
pub mod protocol;
pub mod relay;
pub mod stream;
pub mod transport;

/// Get git hash version of `rust-nostr` libraries
#[uniffi::export]
pub fn git_hash_version() -> Option<String> {
    option_env!("GIT_HASH").map(|v| v.to_string())
}

// Workaround to fix UPX compression error
//
// Error: CantPackException: need DT_INIT; try "void _init(void){}"
// Workaround comes from https://github.com/upx/upx/issues/740
#[unsafe(no_mangle)]
#[cfg(target_os = "android")]
pub fn _init() {}

// Changes to this arg will break binding packages (in particular Swift).
// If this is removed, make sure to update `uniffi.toml`
uniffi::setup_scaffolding!("nostr_sdk");
