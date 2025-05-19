# Changelog

<!-- All notable changes to this project will be documented in this file. -->

<!-- The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), -->
<!-- and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html). -->

<!-- Template

## [Unreleased]

### Breaking changes

### Changed

### Added

### Fixed

### Removed

### Deprecated

-->

## Unreleased

### Changed

* Publish python wheels with cp39-abi3 (https://github.com/rust-nostr/nostr-sdk-ffi/pull/7)

### Added

* Add support for event streaming (https://github.com/rust-nostr/nostr-sdk-ffi/pull/6)
* Add `i686-unknown-linux-gnu`, `i686-pc-windows-msvc` and `aarch64-pc-windows-msvc` support for Python Wheels (https://github.com/rust-nostr/nostr-sdk-ffi/pull/7)
* Add support for `i686-unknown-linux-musl`, `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl` (https://github.com/rust-nostr/nostr-sdk-ffi/pull/8)
* Add support for `armv7-unknown-linux-gnueabihf` and `armv7-unknown-linux-musleabihf` (https://github.com/rust-nostr/nostr-sdk-ffi/pull/9)
* Add support for `riscv64gc-unknown-linux-gnu` and `riscv64gc-unknown-linux-musl` (https://github.com/rust-nostr/nostr-sdk-ffi/pull/10)

## v0.41.0 - 2025/04/15

### Breaking changes

* Remove `TagKind::Clone` and handle as `Unknown` to fix issues with C# bindings

### Changed

* Bump upstream deps to 0.41.0 (see the [Upstream CHANGELOG] for more details)

### Added

* Add support for `i686-pc-windows-msvc` and `aarch64-pc-windows-msvc`
* Add support to `i686-unknown-linux-gnu`
* Expose `Relay::ban`
* Derive `Hash` and `Display` traits where possible

<!-- Links -->
[Upstream CHANGELOG]: https://github.com/rust-nostr/nostr/blob/master/CHANGELOG.md
