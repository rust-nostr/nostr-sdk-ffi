#!/bin/bash

# Compile linux binaries

set -exuo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PYTHON_ENV_PATH="${SCRIPT_DIR}/../venv"

# Create a python env
python -m venv "${PYTHON_ENV_PATH}" || virtualenv "${PYTHON_ENV_PATH}"

# Enter in the python env
. "${PYTHON_ENV_PATH}/bin/activate"

# Install deps
pip install cargo-zigbuild==0.19.8

# Build (GLIBC 2.17)
cargo zigbuild -p nostr-sdk-ffi --target i686-unknown-linux-gnu.2.17 --release
cargo zigbuild -p nostr-sdk-ffi --target x86_64-unknown-linux-gnu.2.17 --release
cargo zigbuild -p nostr-sdk-ffi --target armv7-unknown-linux-gnueabihf.2.17 --release
cargo zigbuild -p nostr-sdk-ffi --target aarch64-unknown-linux-gnu.2.17 --release

# Build (GLIBC 2.29)
cargo zigbuild -p nostr-sdk-ffi --target riscv64gc-unknown-linux-gnu.2.29 --release

# Build (MUSL)
cargo zigbuild -p nostr-sdk-ffi --target i686-unknown-linux-musl --release
cargo zigbuild -p nostr-sdk-ffi --target x86_64-unknown-linux-musl --release
cargo zigbuild -p nostr-sdk-ffi --target armv7-unknown-linux-musleabihf --release
cargo zigbuild -p nostr-sdk-ffi --target aarch64-unknown-linux-musl --release
cargo zigbuild -p nostr-sdk-ffi --target riscv64gc-unknown-linux-musl --release
