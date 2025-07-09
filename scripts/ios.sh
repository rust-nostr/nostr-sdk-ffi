#!/bin/bash

# Compile iOS binaries (works only on a macos machine)

set -exuo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MANIFEST_PATH="${SCRIPT_DIR}/../Cargo.toml"

# Build
cargo build -p nostr-sdk-ffi --manifest-path "${MANIFEST_PATH}" --target aarch64-apple-ios --release
cargo build -p nostr-sdk-ffi --manifest-path "${MANIFEST_PATH}" --target x86_64-apple-ios --release
cargo build -p nostr-sdk-ffi --manifest-path "${MANIFEST_PATH}" --target aarch64-apple-ios-sim --release
