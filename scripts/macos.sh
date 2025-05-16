#!/bin/bash

# Cross compile macOS binaries

set -exuo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MANIFEST_PATH="${SCRIPT_DIR}/../Cargo.toml"
PYTHON_ENV_PATH="${SCRIPT_DIR}/../venv"

# Create a python env
python -m venv "${PYTHON_ENV_PATH}" || virtualenv "${PYTHON_ENV_PATH}"

# Enter in the python env
. "${PYTHON_ENV_PATH}/bin/activate"

# Install deps
pip install cargo-zigbuild==0.19.8

# Build
cargo zigbuild -p nostr-sdk-ffi --manifest-path "${MANIFEST_PATH}" --target universal2-apple-darwin --release
