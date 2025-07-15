#!/bin/bash

# Compile FreeBSD binaries

set -exuo pipefail

# Install deps
cross --version || cargo install cross --git https://github.com/cross-rs/cross --rev 51f46f296253d8122c927c5bb933e3c4f27cc317

# Build
cross build -p nostr-sdk-ffi --target x86_64-unknown-freebsd --release
cargo build -p nostr-sdk-ffi --target aarch64-unknown-freebsd --release
