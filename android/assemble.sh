#!/bin/bash

set -exuo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="${SCRIPT_DIR}/../target"
ANDROID_MAIN_DIR="${SCRIPT_DIR}/lib/src/main"
ANDROID_MAIN_KOTLIN_DIR="${ANDROID_MAIN_DIR}/kotlin"
ANDROID_MAIN_JNI_LIBS_DIR="${ANDROID_MAIN_DIR}/jniLibs"

# Clean
rm -rf "${ANDROID_MAIN_KOTLIN_DIR}"
rm -rf "${ANDROID_MAIN_JNI_LIBS_DIR}"

# Copy binaries
mkdir -p "${ANDROID_MAIN_JNI_LIBS_DIR}/arm64-v8a/"
mkdir -p "${ANDROID_MAIN_JNI_LIBS_DIR}/armeabi-v7a/"
mkdir -p "${ANDROID_MAIN_JNI_LIBS_DIR}/x86/"
mkdir -p "${ANDROID_MAIN_JNI_LIBS_DIR}/x86_64/"
cp "${TARGET_DIR}/aarch64-linux-android/release/libnostr_sdk_ffi.so" "${ANDROID_MAIN_JNI_LIBS_DIR}/arm64-v8a/"
cp "${TARGET_DIR}/armv7-linux-androideabi/release/libnostr_sdk_ffi.so" "${ANDROID_MAIN_JNI_LIBS_DIR}/armeabi-v7a/"
cp "${TARGET_DIR}/i686-linux-android/release/libnostr_sdk_ffi.so" "${ANDROID_MAIN_JNI_LIBS_DIR}/x86/"
cp "${TARGET_DIR}/x86_64-linux-android/release/libnostr_sdk_ffi.so" "${ANDROID_MAIN_JNI_LIBS_DIR}/x86_64"

# Debug
ls -l "${TARGET_DIR}/uniffi/android-kotlin/"

# Copy Kotlin bindings
mkdir -p "${ANDROID_MAIN_KOTLIN_DIR}"
cp -R "${TARGET_DIR}/uniffi/android-kotlin/." "${ANDROID_MAIN_KOTLIN_DIR}"

# Assemble AAR
"${SCRIPT_DIR}/gradlew" assembleRelease
