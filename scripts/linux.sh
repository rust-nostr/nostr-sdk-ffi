#!/bin/bash

# Compile linux binaries

set -exuo pipefail

CDYLIB="libnostr_sdk_ffi.so"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="${SCRIPT_DIR}/../target"
FFI_DIR="${SCRIPT_DIR}/../ffi"
FFI_LINUX_DIR="${FFI_DIR}/linux"
PYTHON_ENV_PATH="${SCRIPT_DIR}/../venv"

# Create a python env
python -m venv "${PYTHON_ENV_PATH}" || virtualenv "${PYTHON_ENV_PATH}"

# Enter in the python env
. "${PYTHON_ENV_PATH}/bin/activate"

# Clean
rm -rf "${FFI_LINUX_DIR}"

# Install deps
pip install cargo-zigbuild==0.19.8

# Build (GLIBC 2.17)
cargo zigbuild -p nostr-sdk-ffi --target i686-unknown-linux-gnu.2.17 --release
cargo zigbuild -p nostr-sdk-ffi --target x86_64-unknown-linux-gnu.2.17 --release
cargo zigbuild -p nostr-sdk-ffi --target aarch64-unknown-linux-gnu.2.17 --release

# Build (MUSL)
cargo zigbuild -p nostr-sdk-ffi --target i686-unknown-linux-musl --release
cargo zigbuild -p nostr-sdk-ffi --target x86_64-unknown-linux-musl --release
cargo zigbuild -p nostr-sdk-ffi --target aarch64-unknown-linux-musl --release

# Make directories
mkdir -p "${FFI_LINUX_DIR}/x86"
mkdir -p "${FFI_LINUX_DIR}/x86_64"
mkdir -p "${FFI_LINUX_DIR}/aarch64"

# Copy dynamic libraries
cp "${TARGET_DIR}/i686-unknown-linux-gnu/release/${CDYLIB}" "${FFI_LINUX_DIR}/x86"
cp "${TARGET_DIR}/x86_64-unknown-linux-gnu/release/${CDYLIB}" "${FFI_LINUX_DIR}/x86_64"
cp "${TARGET_DIR}/aarch64-unknown-linux-gnu/release/${CDYLIB}" "${FFI_LINUX_DIR}/aarch64"
