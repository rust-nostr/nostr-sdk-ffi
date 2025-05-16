#!/bin/bash

set -exuo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MAIN_DIR="${SCRIPT_DIR}/lib/src/main"
KOTLIN_DIR="${MAIN_DIR}/kotlin"
RESOURCE_DIR="${MAIN_DIR}/resources"
TARGET_DIR="${SCRIPT_DIR}/../target"

# Clean
rm -rf "${MAIN_DIR}"

# Copy apple binaries
mkdir -p "${RESOURCE_DIR}/darwin-x86-64/"
mkdir -p "${RESOURCE_DIR}/darwin-aarch64/"
cp "${TARGET_DIR}/x86_64-apple-darwin/release/libnostr_sdk_ffi.dylib" "${RESOURCE_DIR}/darwin-x86-64/"
cp "${TARGET_DIR}/aarch64-apple-darwin/release/libnostr_sdk_ffi.dylib" "${RESOURCE_DIR}/darwin-aarch64/"

# Copy linux glibc binaries
mkdir -p "${RESOURCE_DIR}/linux-x86/"
mkdir -p "${RESOURCE_DIR}/linux-x86-64/"
mkdir -p "${RESOURCE_DIR}/linux-aarch64/"
cp "${TARGET_DIR}/i686-unknown-linux-gnu/release/libnostr_sdk_ffi.so" "${RESOURCE_DIR}/linux-x86/"
cp "${TARGET_DIR}/x86_64-unknown-linux-gnu/release/libnostr_sdk_ffi.so" "${RESOURCE_DIR}/linux-x86-64/"
cp "${TARGET_DIR}/aarch64-unknown-linux-gnu/release/libnostr_sdk_ffi.so" "${RESOURCE_DIR}/linux-aarch64/"

# Copy linux musl binaries
mkdir -p "${RESOURCE_DIR}/linux-musl-x86/"
mkdir -p "${RESOURCE_DIR}/linux-musl-x86-64/"
mkdir -p "${RESOURCE_DIR}/linux-musl-aarch64/"
cp "${TARGET_DIR}/i686-unknown-linux-musl/release/libnostr_sdk_ffi.so" "${RESOURCE_DIR}/linux-musl-x86/"
cp "${TARGET_DIR}/x86_64-unknown-linux-musl/release/libnostr_sdk_ffi.so" "${RESOURCE_DIR}/linux-musl-x86-64/"
cp "${TARGET_DIR}/aarch64-unknown-linux-musl/release/libnostr_sdk_ffi.so" "${RESOURCE_DIR}/linux-musl-aarch64/"

# Copy windows binaries
mkdir -p "${RESOURCE_DIR}/win32-x86/"
mkdir -p "${RESOURCE_DIR}/win32-x86-64/"
mkdir -p "${RESOURCE_DIR}/win32-aarch64/"
cp "${TARGET_DIR}/i686-pc-windows-msvc/release/nostr_sdk_ffi.dll" "${RESOURCE_DIR}/win32-x86/"
cp "${TARGET_DIR}/x86_64-pc-windows-msvc/release/nostr_sdk_ffi.dll" "${RESOURCE_DIR}/win32-x86-64/"
cp "${TARGET_DIR}/aarch64-pc-windows-msvc/release/nostr_sdk_ffi.dll" "${RESOURCE_DIR}/win32-aarch64/"

# Generate Kotlin bindings
cargo run -p nostr-sdk-ffi --features uniffi-cli --bin uniffi-bindgen generate --library "${RESOURCE_DIR}/darwin-x86-64/libnostr_sdk_ffi.dylib" --language kotlin --no-format -o "${KOTLIN_DIR}"

# Build JAR
"${SCRIPT_DIR}/gradlew" jar
