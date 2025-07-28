#!/bin/bash

# Compile FreeBSD binaries

set -exuo pipefail

# Install deps
cross --version || cargo install cross --git https://github.com/cross-rs/cross --rev e281947ca900da425e4ecea7483cfde646c8a1ea

# Build
cross build -p nostr-sdk-ffi --target x86_64-unknown-freebsd --release
