# Changelog

<!-- All notable changes to this project will be documented in this file. -->

<!-- The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), -->
<!-- and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html). -->

<!-- Template

## Unreleased

### Breaking changes

### Changed

### Added

### Fixed

### Removed

### Deprecated

-->

## Unreleased

### Breaking changes

- Convert `AdmitStatus` from enum to object (https://github.com/rust-nostr/nostr-sdk-ffi/pull/29)
- Remove `nip21_extract_from_text` and `Tags::from_text` in favor of `NostrParser`
- Remove getter and setters from `Metadata` object, in favor of `MetadataRecord
- Remove NIP-26 support (as per https://github.com/nostr-protocol/nips/pull/1051/commits/1733dd78b77bb95cde9b18db2671f33870bfcd98)
- Change the relay url arg type around the code from `String` to `RelayUrl` (https://github.com/rust-nostr/nostr-sdk-ffi/pull/28)
- Update `Client::subscriptions` and `Client::subscription` output
- Rename `Options` to `ClientOptions`
- Convert NIP-05, NIP-11 and NIP-96 modules to be I/O-free

### Changed

- Set default params for `EventDeletionRequest` and `Contact`
- Bump nostr from 0.42.0 to 4f4e0429 (see the Upstream CHANGELOG for more details)

### Added

- Expose `NostrParser` (https://github.com/rust-nostr/nostr-sdk-ffi/pull/13)
- Expose arithmetic operations on `Timestamp` with `Duration` (https://github.com/rust-nostr/nostr-sdk-ffi/pull/25)
- Expose `Timestamp::min` and `Timestamp::max`
- Re-expose `CustomNostrDatabase` (https://github.com/rust-nostr/nostr-sdk-ffi/pull/33)
- Add `custom` field to `MetadataRecord`
- Expose `RelayUrl` (https://github.com/rust-nostr/nostr-sdk-ffi/pull/28)
- Add support for `x86_64-unknown-freebsd` (https://github.com/rust-nostr/nostr-sdk-ffi/pull/42)

### Fixed

- Fix NIP22 functions are not exposed

## v0.42.2 - 2025/06/09

### Fixed

- Update the android libraries to use 16KB page alignment (https://github.com/rust-nostr/nostr-sdk-ffi/pull/18)

## v0.42.1 - 2025/05/26

### Changed

- Bump nostr from 0.42.0 to 0.42.1 (see the Upstream CHANGELOG for more details)

## v0.42.0 - 2025/05/20

### Breaking changes

- Rename `Nip46Request` to `NostrConnectRequest` (https://github.com/rust-nostr/nostr-sdk-ffi/pull/11)
- Rename `ExtractedComment` to `CommentTarget` (https://github.com/rust-nostr/nostr-sdk-ffi/pull/11)

### Changed

- Publish python wheels with cp39-abi3 (https://github.com/rust-nostr/nostr-sdk-ffi/pull/7)
- Bump nostr upstream deps to 0.42.0 (see the Upstream CHANGELOG for more details, https://github.com/rust-nostr/nostr-sdk-ffi/pull/11)

### Added

- Add support for event streaming (https://github.com/rust-nostr/nostr-sdk-ffi/pull/6)
- Add `i686-unknown-linux-gnu`, `i686-pc-windows-msvc` and `aarch64-pc-windows-msvc` support for Python Wheels (https://github.com/rust-nostr/nostr-sdk-ffi/pull/7)
- Add support for `i686-unknown-linux-musl`, `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl` (https://github.com/rust-nostr/nostr-sdk-ffi/pull/8)
- Add support for `armv7-unknown-linux-gnueabihf` and `armv7-unknown-linux-musleabihf` (https://github.com/rust-nostr/nostr-sdk-ffi/pull/9)
- Add support for `riscv64gc-unknown-linux-gnu` and `riscv64gc-unknown-linux-musl` (https://github.com/rust-nostr/nostr-sdk-ffi/pull/10)

## v0.41.0 - 2025/04/15

### Breaking changes

- Remove `TagKind::Clone` and handle as `Unknown` to fix issues with C# bindings

### Changed

- Bump upstream deps to 0.41.0 (see the upstream CHANGELOG for more details)

### Added

- Add support for `i686-pc-windows-msvc` and `aarch64-pc-windows-msvc`
- Add support to `i686-unknown-linux-gnu`
- Expose `Relay::ban`
- Derive `Hash` and `Display` traits where possible
