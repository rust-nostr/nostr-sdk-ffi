#!/bin/bash

# Cross compile windows binaries

set -exuo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PYTHON_ENV_PATH="${SCRIPT_DIR}/../venv"

# Create a python env
python -m venv "${PYTHON_ENV_PATH}" || virtualenv "${PYTHON_ENV_PATH}"

# Enter in the python env
. "${PYTHON_ENV_PATH}/bin/activate"

# Install deps
pip install cargo-xwin==0.18.4

# Build
cargo xwin build -p nostr-sdk-ffi --target i686-pc-windows-msvc --release --cross-compiler clang
cargo xwin build -p nostr-sdk-ffi --target x86_64-pc-windows-msvc --release
cargo xwin build -p nostr-sdk-ffi --target aarch64-pc-windows-msvc --release --cross-compiler clang
