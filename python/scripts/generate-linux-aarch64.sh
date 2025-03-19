#!/usr/bin/env bash

set -exuo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="${SCRIPT_DIR}/../../target"
PYTHON_SRC_DIR="${SCRIPT_DIR}/../src"

python --version
pip install -r "${SCRIPT_DIR}/../requirements.txt"

echo "Generating native binaries..."
rustup target add aarch64-unknown-linux-gnu
CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc cargo build --lib --release --target aarch64-unknown-linux-gnu

echo "Generating nostr_sdk.py..."
cargo run --features uniffi-cli --bin uniffi-bindgen generate --library "${TARGET_DIR}/aarch64-unknown-linux-gnu/release/libnostr_sdk_ffi.so" --language python --no-format -o "${PYTHON_SRC_DIR}/nostr-sdk/"

echo "Copying linux libnostr_sdk_ffi.so..."
cp "${TARGET_DIR}/aarch64-unknown-linux-gnu/release/libnostr_sdk_ffi.so" "${PYTHON_SRC_DIR}/nostr-sdk/"

echo "All done!"
