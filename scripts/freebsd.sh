#!/bin/bash

# Compile FreeBSD binaries

set -exuo pipefail

# Install deps
cross --version || cargo install cross

# Build
cross build -p nostr-sdk-ffi --target x86_64-unknown-freebsd --release
