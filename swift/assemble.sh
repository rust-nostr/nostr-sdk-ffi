#!/bin/bash

set -exuo pipefail

NAME="nostr_sdkFFI"
PKG_NAME="NostrSDK"
STATIC_LIB="libnostr_sdk_ffi.a"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="${SCRIPT_DIR}/../target"
FFI_SWIFT_DIR="${SCRIPT_DIR}/uniffi-output"
FFI_HEADERS_DIR="${FFI_SWIFT_DIR}/include"
FFI_SOURCES_DIR="${FFI_SWIFT_DIR}/Sources"
SOURCES_DIR="${SCRIPT_DIR}/Sources"
XCFRAMEWORK_DIR="${SCRIPT_DIR}/${NAME}.xcframework"

# Clean
rm -rf "${SOURCES_DIR}"     # Remove old Sources dir
rm -rf "${XCFRAMEWORK_DIR}" # Remove old <NAME>.xcframework dir

# Generate Swift bindings
cargo run -p nostr-sdk-ffi --bin uniffi-bindgen generate --library "${TARGET_DIR}/aarch64-apple-ios/release/${STATIC_LIB}" --no-format --language swift --out-dir "${FFI_SWIFT_DIR}"

# Current `FFI_SWIFT_DIR` structure (output of UniFFI):
# -rw-r--r-- nostr_sdkFFI.h
# -rw-r--r-- nostr_sdkFFI.modulemap
# -rw-r--r-- nostr_sdk.swift
#
# Steps to reorganize the FFI dir:
# - `nostr_sdkFFI.h` must be moved in `uniffi-output/include` dir
# - `nostr_sdkFFI.modulemap` must be renamed to `module.modulemap` and moved to `uniffi-output/include` dir
# - `nostr_sdk.swift` must be renamed to NostrSDK.swift and moved to `uniffi-output/Sources/NostrSDK` dir
#
# New expected `FFI_SWIFT_DIR` structure:
# .
# ├── include
# │   ├── module.modulemap
# │   └── nostr_sdkFFI.h
# └── Sources
#     └── NostrSDK
#         └── NostrSDK.swift

mkdir -p "${FFI_HEADERS_DIR}"                                                               # Make `uniffi-output/include` dir
mkdir -p "${FFI_SOURCES_DIR}/${PKG_NAME}"                                                   # Make `uniffi-output/Sources/NostrSDK` dir
mv "${FFI_SWIFT_DIR}/${NAME}.h" "${FFI_HEADERS_DIR}/${NAME}.h"                              # Move header to `include` dir
mv "${FFI_SWIFT_DIR}/${NAME}.modulemap" "${FFI_HEADERS_DIR}/module.modulemap"               # Rename and move modulemap
mv "${FFI_SWIFT_DIR}/nostr_sdk.swift" "${FFI_SOURCES_DIR}/${PKG_NAME}/${PKG_NAME}.swift"    # Rename and move swift file

# Copy `uniffi-output/Sources` dir to the Swift package
cp -r "${FFI_SOURCES_DIR}" "${SOURCES_DIR}"

# Create new xcframework from static libs and headers
xcodebuild -create-xcframework \
    -library "${TARGET_DIR}/aarch64-apple-ios/release/${STATIC_LIB}" \
    -headers "${FFI_HEADERS_DIR}" \
    -library "${TARGET_DIR}/ios-universal-sim/release/${STATIC_LIB}" \
    -headers "${FFI_HEADERS_DIR}" \
    -library "${TARGET_DIR}/darwin-universal/release/${STATIC_LIB}" \
    -headers "${FFI_HEADERS_DIR}" \
    -library "${TARGET_DIR}/catalyst-universal/release/${STATIC_LIB}" \
    -headers "${FFI_HEADERS_DIR}" \
    -output "${XCFRAMEWORK_DIR}"
