#!/bin/bash

set -exuo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="${SCRIPT_DIR}/../target"
UNIFFI_CONFIG_PATH="${SCRIPT_DIR}/../uniffi-kmp.toml"
FFI_DIR="${SCRIPT_DIR}/../ffi"
FFI_KMP_DIR="${FFI_DIR}/kmp"
FFI_ANDROID_JNI_LIBS_DIR="${FFI_DIR}/kotlin-android/jniLibs"
SRC_DIR="${SCRIPT_DIR}/nostr-sdk-kmp/src"
ANDROID_MAIN_DIR="${SRC_DIR}/androidMain"
ANDROID_MAIN_JNI_LIBS_DIR="${ANDROID_MAIN_DIR}/jniLibs"
COMMON_MAIN_DIR="${SRC_DIR}/commonMain"
NATIVE_MAIN_DIR="${SRC_DIR}/nativeMain"
JVM_MAIN_DIR="${SRC_DIR}/jvmMain"

# Clean
rm -rf "${FFI_KMP_DIR}"
rm -rf "${ANDROID_MAIN_DIR}"
rm -rf "${COMMON_MAIN_DIR}"
rm -rf "${NATIVE_MAIN_DIR}"
rm -rf "${JVM_MAIN_DIR}"
rm -rf "${SRC_DIR}/nativeInterop/cinterop/headers"

# Install deps
gobley-uniffi-bindgen --version || cargo install gobley-uniffi-bindgen --git https://github.com/gobley/gobley --tag v0.2.0

# Copy android binaries
mkdir -p "${ANDROID_MAIN_JNI_LIBS_DIR}"
cp -r "${FFI_ANDROID_JNI_LIBS_DIR}" "${ANDROID_MAIN_JNI_LIBS_DIR}"

# Copy iOS binaries
#mkdir -p "${SRC_DIR}/lib/ios-arm64/"
#mkdir -p "${SRC_DIR}/lib/ios-simulator-arm64/"
#mkdir -p "${SRC_DIR}/lib/ios-simulator-x64/"
#cp "${TARGET_DIR}/aarch64-apple-ios/release/libnostr_sdk_ffi.a" "${SRC_DIR}/lib/ios-arm64/"
#cp "${TARGET_DIR}/aarch64-apple-ios-sim/release/libnostr_sdk_ffi.a" "${SRC_DIR}/lib/ios-simulator-arm64/"
#cp "${TARGET_DIR}/x86_64-apple-ios/release/libnostr_sdk_ffi.a" "${SRC_DIR}/lib/ios-simulator-x64/"

# Copy apple binaries
mkdir -p "${JVM_MAIN_DIR}/resources/darwin-x86-64/"
mkdir -p "${JVM_MAIN_DIR}/resources/darwin-aarch64/"
cp "${TARGET_DIR}/x86_64-apple-darwin/release/libnostr_sdk_ffi.dylib" "${JVM_MAIN_DIR}/resources/darwin-x86-64/"
cp "${TARGET_DIR}/aarch64-apple-darwin/release/libnostr_sdk_ffi.dylib" "${JVM_MAIN_DIR}/resources/darwin-aarch64/"

# Copy linux glibc binaries
mkdir -p "${JVM_MAIN_DIR}/resources/linux-x86/"
mkdir -p "${JVM_MAIN_DIR}/resources/linux-x86-64/"
mkdir -p "${JVM_MAIN_DIR}/resources/linux-arm/"
mkdir -p "${JVM_MAIN_DIR}/resources/linux-aarch64/"
mkdir -p "${JVM_MAIN_DIR}/resources/linux-riscv64/"
cp "${TARGET_DIR}/i686-unknown-linux-gnu/release/libnostr_sdk_ffi.so" "${JVM_MAIN_DIR}/resources/linux-x86/"
cp "${TARGET_DIR}/x86_64-unknown-linux-gnu/release/libnostr_sdk_ffi.so" "${JVM_MAIN_DIR}/resources/linux-x86-64/"
cp "${TARGET_DIR}/armv7-unknown-linux-gnueabihf/release/libnostr_sdk_ffi.so" "${JVM_MAIN_DIR}/resources/linux-arm/"
cp "${TARGET_DIR}/aarch64-unknown-linux-gnu/release/libnostr_sdk_ffi.so" "${JVM_MAIN_DIR}/resources/linux-aarch64/"
cp "${TARGET_DIR}/riscv64gc-unknown-linux-gnu/release/libnostr_sdk_ffi.so" "${JVM_MAIN_DIR}/resources/linux-riscv64/"

# Copy linux musl binaries
mkdir -p "${JVM_MAIN_DIR}/resources/linux-x86-musl/"
mkdir -p "${JVM_MAIN_DIR}/resources/linux-x86-64-musl/"
mkdir -p "${JVM_MAIN_DIR}/resources/linux-arm-musl/"
mkdir -p "${JVM_MAIN_DIR}/resources/linux-aarch64-musl/"
mkdir -p "${JVM_MAIN_DIR}/resources/linux-riscv64-musl/"
cp "${TARGET_DIR}/i686-unknown-linux-musl/release/libnostr_sdk_ffi.so" "${JVM_MAIN_DIR}/resources/linux-x86-musl/"
cp "${TARGET_DIR}/x86_64-unknown-linux-musl/release/libnostr_sdk_ffi.so" "${JVM_MAIN_DIR}/resources/linux-x86-64-musl/"
cp "${TARGET_DIR}/armv7-unknown-linux-musleabihf/release/libnostr_sdk_ffi.so" "${JVM_MAIN_DIR}/resources/linux-arm-musl/"
cp "${TARGET_DIR}/aarch64-unknown-linux-musl/release/libnostr_sdk_ffi.so" "${JVM_MAIN_DIR}/resources/linux-aarch64-musl/"
cp "${TARGET_DIR}/riscv64gc-unknown-linux-musl/release/libnostr_sdk_ffi.so" "${JVM_MAIN_DIR}/resources/linux-riscv64-musl/"

# Copy windows binaries
mkdir -p "${JVM_MAIN_DIR}/resources/win32-x86/"
mkdir -p "${JVM_MAIN_DIR}/resources/win32-x86-64/"
mkdir -p "${JVM_MAIN_DIR}/resources/win32-aarch64/"
cp "${TARGET_DIR}/i686-pc-windows-msvc/release/nostr_sdk_ffi.dll" "${JVM_MAIN_DIR}/resources/win32-x86/"
cp "${TARGET_DIR}/x86_64-pc-windows-msvc/release/nostr_sdk_ffi.dll" "${JVM_MAIN_DIR}/resources/win32-x86-64/"
cp "${TARGET_DIR}/aarch64-pc-windows-msvc/release/nostr_sdk_ffi.dll" "${JVM_MAIN_DIR}/resources/win32-aarch64/"

# Generate bindings
gobley-uniffi-bindgen -c "${UNIFFI_CONFIG_PATH}" --library "${JVM_MAIN_DIR}/resources/darwin-x86-64/libnostr_sdk_ffi.dylib" -o "${FFI_KMP_DIR}"

# Copy bindings
mkdir -p "${ANDROID_MAIN_DIR}/kotlin/rust/nostr/sdk/"
mkdir -p "${COMMON_MAIN_DIR}/kotlin/rust/nostr/sdk/"
mkdir -p "${NATIVE_MAIN_DIR}/kotlin/rust/nostr/sdk/"
mkdir -p "${JVM_MAIN_DIR}/kotlin/rust/nostr/sdk/"
cp -r "${FFI_KMP_DIR}/nativeInterop" "${SRC_DIR}"
cp "${FFI_KMP_DIR}/main/kotlin/rust/nostr/sdk/nostr_sdk.android.kt" "${ANDROID_MAIN_DIR}/kotlin/rust/nostr/sdk/"
cp "${FFI_KMP_DIR}/main/kotlin/rust/nostr/sdk/nostr_sdk.common.kt" "${COMMON_MAIN_DIR}/kotlin/rust/nostr/sdk/"
cp "${FFI_KMP_DIR}/main/kotlin/rust/nostr/sdk/nostr_sdk.native.kt" "${NATIVE_MAIN_DIR}/kotlin/rust/nostr/sdk/"
cp "${FFI_KMP_DIR}/main/kotlin/rust/nostr/sdk/nostr_sdk.jvm.kt" "${JVM_MAIN_DIR}/kotlin/rust/nostr/sdk/"

# Assemble
./gradlew :nostr-sdk-kmp:assemble
