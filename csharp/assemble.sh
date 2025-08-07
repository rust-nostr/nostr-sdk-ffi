#!/bin/bash

set -exuo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MAIN_DIR="${SCRIPT_DIR}/Nostr.Sdk"
RUNTIME_DIR="${MAIN_DIR}/runtimes"
TARGET_DIR="${SCRIPT_DIR}/../target"

# Clean
rm -rf "${RUNTIME_DIR}"
rm -rf "${MAIN_DIR}/nostr_sdk.cs"

# Copy macOS binaries
mkdir -p "${RUNTIME_DIR}/osx-x64/native/"
mkdir -p "${RUNTIME_DIR}/osx-arm64/native/"
cp "${TARGET_DIR}/x86_64-apple-darwin/release/libnostr_sdk_ffi.dylib" "${RUNTIME_DIR}/osx-x64/native/"
cp "${TARGET_DIR}/aarch64-apple-darwin/release/libnostr_sdk_ffi.dylib" "${RUNTIME_DIR}/osx-arm64/native/"

# Copy linux glibc binaries
mkdir -p "${RUNTIME_DIR}/linux-x86/native/"
mkdir -p "${RUNTIME_DIR}/linux-x64/native/"
mkdir -p "${RUNTIME_DIR}/linux-arm/native/"
mkdir -p "${RUNTIME_DIR}/linux-arm64/native/"
mkdir -p "${RUNTIME_DIR}/linux-riscv64/native/"
cp "${TARGET_DIR}/i686-unknown-linux-gnu/release/libnostr_sdk_ffi.so" "${RUNTIME_DIR}/linux-x86/native/"
cp "${TARGET_DIR}/x86_64-unknown-linux-gnu/release/libnostr_sdk_ffi.so" "${RUNTIME_DIR}/linux-x64/native/"
cp "${TARGET_DIR}/armv7-unknown-linux-gnueabihf/release/libnostr_sdk_ffi.so" "${RUNTIME_DIR}/linux-arm/native/"
cp "${TARGET_DIR}/aarch64-unknown-linux-gnu/release/libnostr_sdk_ffi.so" "${RUNTIME_DIR}/linux-arm64/native/"
cp "${TARGET_DIR}/riscv64gc-unknown-linux-gnu/release/libnostr_sdk_ffi.so" "${RUNTIME_DIR}/linux-riscv64/native/"

# Copy linux musl binaries
mkdir -p "${RUNTIME_DIR}/linux-musl-x86/native/"
mkdir -p "${RUNTIME_DIR}/linux-musl-x64/native/"
mkdir -p "${RUNTIME_DIR}/linux-musl-arm/native/"
mkdir -p "${RUNTIME_DIR}/linux-musl-arm64/native/"
mkdir -p "${RUNTIME_DIR}/linux-musl-riscv64/native/"
cp "${TARGET_DIR}/i686-unknown-linux-musl/release/libnostr_sdk_ffi.so" "${RUNTIME_DIR}/linux-musl-x86/native/"
cp "${TARGET_DIR}/x86_64-unknown-linux-musl/release/libnostr_sdk_ffi.so" "${RUNTIME_DIR}/linux-musl-x64/native/"
cp "${TARGET_DIR}/armv7-unknown-linux-musleabihf/release/libnostr_sdk_ffi.so" "${RUNTIME_DIR}/linux-musl-arm/native/"
cp "${TARGET_DIR}/aarch64-unknown-linux-musl/release/libnostr_sdk_ffi.so" "${RUNTIME_DIR}/linux-musl-arm64/native/"
cp "${TARGET_DIR}/riscv64gc-unknown-linux-musl/release/libnostr_sdk_ffi.so" "${RUNTIME_DIR}/linux-musl-riscv64/native/"

# Copy FreeBSD binaries
mkdir -p "${RUNTIME_DIR}/freebsd-x64/native/"
cp "${TARGET_DIR}/x86_64-unknown-freebsd/release/libnostr_sdk_ffi.so" "${RUNTIME_DIR}/freebsd-x64/native/"

# Copy windows binaries
mkdir -p "${RUNTIME_DIR}/win-x86/native/"
mkdir -p "${RUNTIME_DIR}/win-x64/native/"
mkdir -p "${RUNTIME_DIR}/win-arm64/native/"
cp "${TARGET_DIR}/i686-pc-windows-msvc/release/nostr_sdk_ffi.dll" "${RUNTIME_DIR}/win-x86/native/"
cp "${TARGET_DIR}/x86_64-pc-windows-msvc/release/nostr_sdk_ffi.dll" "${RUNTIME_DIR}/win-x64/native/"
cp "${TARGET_DIR}/aarch64-pc-windows-msvc/release/nostr_sdk_ffi.dll" "${RUNTIME_DIR}/win-arm64/native/"

# Debug
ls -l "${TARGET_DIR}/uniffi/csharp/"

# Copy bindings
cp "${TARGET_DIR}/uniffi/csharp/nostr_sdk.cs" "${MAIN_DIR}"

# Change dir to src
cd "${MAIN_DIR}"

# Build
dotnet build Nostr.Sdk.csproj

# Pack
dotnet pack --configuration Release Nostr.Sdk.csproj

echo
echo "NuPkg located at ${MAIN_DIR}/bin/Release/"
echo
