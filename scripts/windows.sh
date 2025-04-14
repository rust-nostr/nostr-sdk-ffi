#!/bin/bash

# Cross compile windows binaries

set -exuo pipefail

CDYLIB="nostr_sdk_ffi.dll"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="${SCRIPT_DIR}/../target"
FFI_DIR="${SCRIPT_DIR}/../ffi"
FFI_WIN_DIR="${FFI_DIR}/win"
PYTHON_ENV_PATH="${SCRIPT_DIR}/../venv"

# Create a python env
python -m venv "${PYTHON_ENV_PATH}" || virtualenv "${PYTHON_ENV_PATH}"

# Enter in the python env
. "${PYTHON_ENV_PATH}/bin/activate"

# Clean
rm -rf "${FFI_WIN_DIR}"

# Install deps
pip install cargo-xwin==0.18.4

# Build
cargo xwin build -p nostr-sdk-ffi --target x86_64-pc-windows-msvc --release
cargo xwin build -p nostr-sdk-ffi --target aarch64-pc-windows-msvc --release --cross-compiler clang

# Make directories
mkdir -p "${FFI_WIN_DIR}/x86_64"
mkdir -p "${FFI_WIN_DIR}/aarch64"

# Copy binaries
cp "${TARGET_DIR}/x86_64-pc-windows-msvc/release/${CDYLIB}" "${FFI_WIN_DIR}/x86_64"
cp "${TARGET_DIR}/aarch64-pc-windows-msvc/release/${CDYLIB}" "${FFI_WIN_DIR}/aarch64"
