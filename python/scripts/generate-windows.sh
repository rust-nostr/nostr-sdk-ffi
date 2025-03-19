#!/usr/bin/env bash

set -exuo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="${SCRIPT_DIR}/../../target"
PYTHON_SRC_DIR="${SCRIPT_DIR}/../src"

python3 --version
pip install --user -r "${SCRIPT_DIR}/../requirements.txt"

echo "Generating native binaries..."
rustup target add x86_64-pc-windows-msvc
cargo build --lib --release --target x86_64-pc-windows-msvc

echo "Generating nostr_sdk.py..."
cargo run --features uniffi-cli --bin uniffi-bindgen generate --library "${TARGET_DIR}/x86_64-pc-windows-msvc/release/nostr_sdk_ffi.dll" --language python --no-format -o "${PYTHON_SRC_DIR}/nostr-sdk/"

echo "Copying libraries nostr_sdk_ffi.dll..."
cp "${TARGET_DIR}/x86_64-pc-windows-msvc/release/nostr_sdk_ffi.dll" "${PYTHON_SRC_DIR}/nostr-sdk/"

echo "All done!"
