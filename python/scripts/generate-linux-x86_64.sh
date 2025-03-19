#!/usr/bin/env bash

set -exuo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="${SCRIPT_DIR}/../../target"
PYTHON_SRC_DIR="${SCRIPT_DIR}/../src"

${PYBIN}/python --version
${PYBIN}/pip install -r "${SCRIPT_DIR}/../requirements.txt"

echo "Generating native binaries..."
rustup target add x86_64-unknown-linux-gnu
cargo build --lib --release --target x86_64-unknown-linux-gnu

echo "Generating nostr_sdk.py..."
cargo run --features uniffi-cli --bin uniffi-bindgen generate --library "${TARGET_DIR}/x86_64-unknown-linux-gnu/release/libnostr_sdk_ffi.so" --language python --no-format -o "${PYTHON_SRC_DIR}/nostr-sdk/"

echo "Copying linux libnostr_sdk_ffi.so..."
cp "${TARGET_DIR}/x86_64-unknown-linux-gnu/release/libnostr_sdk_ffi.so" "${PYTHON_SRC_DIR}/nostr-sdk/"

echo "All done!"
