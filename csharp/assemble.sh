#!/bin/bash

set -exuo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MAIN_DIR="${SCRIPT_DIR}/Nostr.Sdk"
RUNTIME_DIR="${MAIN_DIR}/runtimes"
FFI_DIR="${SCRIPT_DIR}/../ffi"

# Clean
rm -rf "${RUNTIME_DIR}"

# Install deps
uniffi-bindgen-cs --version || cargo install uniffi-bindgen-cs --git https://github.com/NordSecurity/uniffi-bindgen-cs --tag v0.9.0+v0.28.3

# Copy apple binaries
mkdir -p "${RUNTIME_DIR}/osx-x64/native/"
mkdir -p "${RUNTIME_DIR}/osx-arm64/native/"
cp "${FFI_DIR}/apple/macos/x86_64/libnostr_sdk_ffi.dylib" "${RUNTIME_DIR}/osx-x64/native/"
cp "${FFI_DIR}/apple/macos/aarch64/libnostr_sdk_ffi.dylib" "${RUNTIME_DIR}/osx-arm64/native/"

# Copy linux binaries
mkdir -p "${RUNTIME_DIR}/linux-x64/native/"
mkdir -p "${RUNTIME_DIR}/linux-arm64/native/"
cp "${FFI_DIR}/linux/x86_64/libnostr_sdk_ffi.so" "${RUNTIME_DIR}/linux-x64/native/"
cp "${FFI_DIR}/linux/aarch64/libnostr_sdk_ffi.so" "${RUNTIME_DIR}/linux-arm64/native/"

# Copy windows binaries
mkdir -p "${RUNTIME_DIR}/win-x86/native/"
mkdir -p "${RUNTIME_DIR}/win-x64/native/"
mkdir -p "${RUNTIME_DIR}/win-arm64/native/"
cp "${FFI_DIR}/win/x86/nostr_sdk_ffi.dll" "${RUNTIME_DIR}/win-x86/native/"
cp "${FFI_DIR}/win/x86_64/nostr_sdk_ffi.dll" "${RUNTIME_DIR}/win-x64/native/"
cp "${FFI_DIR}/win/aarch64/nostr_sdk_ffi.dll" "${RUNTIME_DIR}/win-arm64/native/"

# Generate bindings
uniffi-bindgen-cs --library "${RUNTIME_DIR}/osx-x64/native/libnostr_sdk_ffi.dylib" -o "${MAIN_DIR}"

# Change dir to src
cd "${MAIN_DIR}"

# Build
dotnet build Nostr.Sdk.csproj

# Pack
dotnet pack --configuration Release Nostr.Sdk.csproj

echo
echo "NuPkg located at ${MAIN_DIR}/bin/Release/"
echo
