#!/usr/bin/env bash

set -exuo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="${SCRIPT_DIR}/../../target"
PYTHON_SRC_DIR="${SCRIPT_DIR}/../src"

python3 --version
pip install --user -r "${SCRIPT_DIR}/../requirements.txt"

echo "Generating native binaries..."
rustup target add aarch64-apple-darwin
cargo build --lib --release --target aarch64-apple-darwin

echo "Generating nostr_sdk.py..."
cargo run --features uniffi-cli --bin uniffi-bindgen generate --library "${TARGET_DIR}/aarch64-apple-darwin/release/libnostr_sdk_ffi.dylib" --language python --no-format -o "${PYTHON_SRC_DIR}/nostr-sdk/"

echo "Copying libraries libnostr_sdk_ffi.dylib..."
cp "${TARGET_DIR}/aarch64-apple-darwin/release/libnostr_sdk_ffi.dylib" "${PYTHON_SRC_DIR}/nostr-sdk/"

echo "All done!"
