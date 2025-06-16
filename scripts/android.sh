#!/bin/bash

# Build android binaries and generate kotlin (android) foreign languages

set -exuo pipefail

CDYLIB="libnostr_sdk_ffi.so"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="${SCRIPT_DIR}/../target"
UNIFFI_CONFIG_PATH="${SCRIPT_DIR}/../uniffi-android.toml"
FFI_DIR="${SCRIPT_DIR}/../ffi"
FFI_KOTLIN_DIR="${FFI_DIR}/kotlin-android"
FFI_JNI_LIBS_DIR="${FFI_KOTLIN_DIR}/jniLibs"
FFI_OUTPUT_DIR="${FFI_DIR}/aar"

# Check if ANDROID_NDK_HOME env is set
if [ ! -d "${ANDROID_NDK_HOME}" ] ; then \
  echo "Error: Please, set the ANDROID_NDK_HOME env variable to point to your NDK folder" ; \
  exit 1 ; \
fi

# Check if ANDROID_SDK_ROOT env is set
if [ ! -d "${ANDROID_SDK_ROOT}" ] ; then \
  echo "Error: Please, set the ANDROID_SDK_ROOT env variable to point to your SDK folder" ; \
  exit 1 ; \
fi

# Install deps
cargo ndk --version || cargo install cargo-ndk

# Clean
rm -rf "${FFI_KOTLIN_DIR}"
rm -rf "${FFI_OUTPUT_DIR}"

# Build targets
cargo ndk -t aarch64-linux-android -t armv7-linux-androideabi -t x86_64-linux-android -t i686-linux-android -o "${FFI_JNI_LIBS_DIR}" build -p nostr-sdk-ffi --lib --release

# Generate Kotlin bindings
cargo run -p nostr-sdk-ffi --features uniffi-cli --bin uniffi-bindgen generate --library "${TARGET_DIR}/aarch64-linux-android/release/${CDYLIB}" --config "${UNIFFI_CONFIG_PATH}" --language kotlin --no-format -o "${FFI_KOTLIN_DIR}"
