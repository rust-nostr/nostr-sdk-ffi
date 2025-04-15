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

## v0.41.0 - 2025/04/15

### Breaking changes

* Remove `TagKind::Clone` and handle as `Unknown` to fix issues with C# bindings

### Changed

* Bump upstream deps to 0.41.0 (see the [Upstream CHANGELOG] for more details)

### Added

* Add support for `i686-pc-windows-msvc` and `aarch64-pc-windows-msv`
* Add support to `i686-unknown-linux-gnu`
* Expose `Relay::ban`
* Derive `Hash` and `Display` traits where possible

<!-- Links -->
[Upstream CHANGELOG]: https://github.com/rust-nostr/nostr/blob/master/CHANGELOG.md
