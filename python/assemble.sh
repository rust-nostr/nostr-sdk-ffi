#!/bin/bash

set -exuo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DIST_DIR="${SCRIPT_DIR}/dist"
SRC_DIR="${SCRIPT_DIR}/src/nostr-sdk"
FFI_DIR="${SCRIPT_DIR}/../ffi"

# Clean
rm -rf "${DIST_DIR}"
rm -rf "${SRC_DIR}/*.so"
rm -rf "${SRC_DIR}/nostr_sdk.py"

# Make dir
mkdir -p "${DIST_DIR}"

# Build docker image
docker build -t wheel-builder "${SCRIPT_DIR}"

# Generate bindings
cargo run -p nostr-sdk-ffi --features uniffi-cli --bin uniffi-bindgen generate --library "${FFI_DIR}/apple/macos/x86_64/libnostr_sdk_ffi.dylib" --language python --no-format -o "${SRC_DIR}"

# Build linux wheels
docker run --rm -v "${FFI_DIR}/linux/x86/:/build/binaries" -v "${SRC_DIR}:/build/binding" -v "$(pwd)/dist:/build/dist" -e PLAT_NAME="manylinux_2_17_i686" wheel-builder
docker run --rm -v "${FFI_DIR}/linux/x86_64/:/build/binaries" -v "${SRC_DIR}:/build/binding" -v "$(pwd)/dist:/build/dist" -e PLAT_NAME="manylinux_2_17_x86_64" wheel-builder
docker run --rm -v "${FFI_DIR}/linux/aarch64/:/build/binaries" -v "${SRC_DIR}:/build/binding" -v "$(pwd)/dist:/build/dist" -e PLAT_NAME="manylinux_2_17_aarch64" wheel-builder

# Build macos wheels
docker run --rm -v "${FFI_DIR}/apple/macos/x86_64/:/build/binaries" -v "${SRC_DIR}:/build/binding" -v "$(pwd)/dist:/build/dist" -e PLAT_NAME="macosx_11_0_x86_64" wheel-builder
docker run --rm -v "${FFI_DIR}/apple/macos/aarch64/:/build/binaries" -v "${SRC_DIR}:/build/binding" -v "$(pwd)/dist:/build/dist" -e PLAT_NAME="macosx_11_0_arm64" wheel-builder

# Build win wheels
docker run --rm -v "${FFI_DIR}/win/x86/:/build/binaries" -v "${SRC_DIR}:/build/binding" -v "$(pwd)/dist:/build/dist" -e PLAT_NAME="win32" wheel-builder
docker run --rm -v "${FFI_DIR}/win/x86_64/:/build/binaries" -v "${SRC_DIR}:/build/binding" -v "$(pwd)/dist:/build/dist" -e PLAT_NAME="win_amd64" wheel-builder
docker run --rm -v "${FFI_DIR}/win/aarch64/:/build/binaries" -v "${SRC_DIR}:/build/binding" -v "$(pwd)/dist:/build/dist" -e PLAT_NAME="win_arm64" wheel-builder
