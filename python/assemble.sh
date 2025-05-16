#!/bin/bash

set -exuo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DIST_DIR="${SCRIPT_DIR}/dist"
SRC_DIR="${SCRIPT_DIR}/src/nostr-sdk"
TARGET_DIR="${SCRIPT_DIR}/../target"

# Clean
rm -rf "${DIST_DIR}"
rm -rf "${SRC_DIR}/*.so"
rm -rf "${SRC_DIR}/nostr_sdk.py"

# Make dir
mkdir -p "${DIST_DIR}"

# Build docker image
docker build -t wheel-builder "${SCRIPT_DIR}"

# Generate bindings
cargo run -p nostr-sdk-ffi --features uniffi-cli --bin uniffi-bindgen generate --library "${TARGET_DIR}/x86_64-apple-darwin/release/libnostr_sdk_ffi.dylib" --language python --no-format -o "${SRC_DIR}"

# Build linux glibc wheels
docker run --rm -v "${TARGET_DIR}/i686-unknown-linux-gnu/release/libnostr_sdk_ffi.so:/build/binaries/libnostr_sdk_ffi.so" -v "${SRC_DIR}:/build/binding" -v "$(pwd)/dist:/build/dist" -e PLAT_NAME="manylinux_2_17_i686" wheel-builder
docker run --rm -v "${TARGET_DIR}/x86_64-unknown-linux-gnu/release/libnostr_sdk_ffi.so:/build/binaries/libnostr_sdk_ffi.so" -v "${SRC_DIR}:/build/binding" -v "$(pwd)/dist:/build/dist" -e PLAT_NAME="manylinux_2_17_x86_64" wheel-builder
docker run --rm -v "${TARGET_DIR}/aarch64-unknown-linux-gnu/release/libnostr_sdk_ffi.so:/build/binaries/libnostr_sdk_ffi.so" -v "${SRC_DIR}:/build/binding" -v "$(pwd)/dist:/build/dist" -e PLAT_NAME="manylinux_2_17_aarch64" wheel-builder

# Build linux musl wheels
docker run --rm -v "${TARGET_DIR}/i686-unknown-linux-musl/release/libnostr_sdk_ffi.a:/build/binaries/libnostr_sdk_ffi.a" -v "${SRC_DIR}:/build/binding" -v "$(pwd)/dist:/build/dist" -e PLAT_NAME="musllinux_1_2_i686" wheel-builder
docker run --rm -v "${TARGET_DIR}/x86_64-unknown-linux-musl/release/libnostr_sdk_ffi.a:/build/binaries/libnostr_sdk_ffi.a" -v "${SRC_DIR}:/build/binding" -v "$(pwd)/dist:/build/dist" -e PLAT_NAME="musllinux_1_2_x86_64" wheel-builder
docker run --rm -v "${TARGET_DIR}/aarch64-unknown-linux-musl/release/libnostr_sdk_ffi.a:/build/binaries/libnostr_sdk_ffi.a" -v "${SRC_DIR}:/build/binding" -v "$(pwd)/dist:/build/dist" -e PLAT_NAME="musllinux_1_2_aarch64" wheel-builder

# Build macos wheels
docker run --rm -v "${TARGET_DIR}/x86_64-apple-darwin/release/libnostr_sdk_ffi.dylib:/build/binaries/libnostr_sdk_ffi.dylib" -v "${SRC_DIR}:/build/binding" -v "$(pwd)/dist:/build/dist" -e PLAT_NAME="macosx_11_0_x86_64" wheel-builder
docker run --rm -v "${TARGET_DIR}/aarch64-apple-darwin/release/libnostr_sdk_ffi.dylib:/build/binaries/libnostr_sdk_ffi.dylib" -v "${SRC_DIR}:/build/binding" -v "$(pwd)/dist:/build/dist" -e PLAT_NAME="macosx_11_0_arm64" wheel-builder

# Build win wheels
docker run --rm -v "${TARGET_DIR}/i686-pc-windows-msvc/release/nostr_sdk_ffi.dll:/build/binaries/nostr_sdk_ffi.dll" -v "${SRC_DIR}:/build/binding" -v "$(pwd)/dist:/build/dist" -e PLAT_NAME="win32" wheel-builder
docker run --rm -v "${TARGET_DIR}/x86_64-pc-windows-msvc/release/nostr_sdk_ffi.dll:/build/binaries/nostr_sdk_ffi.dll" -v "${SRC_DIR}:/build/binding" -v "$(pwd)/dist:/build/dist" -e PLAT_NAME="win_amd64" wheel-builder
docker run --rm -v "${TARGET_DIR}/aarch64-pc-windows-msvc/release/nostr_sdk_ffi.dll:/build/binaries/nostr_sdk_ffi.dll" -v "${SRC_DIR}:/build/binding" -v "$(pwd)/dist:/build/dist" -e PLAT_NAME="win_arm64" wheel-builder
