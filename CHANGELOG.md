# Changelog

<!-- All notable changes to this project will be documented in this file. -->

<!-- The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), -->
<!-- and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html). -->

<!-- Template

## [Unreleased]

### Summary

### Breaking changes

### Changed

### Added

### Fixed

### Removed

### Deprecated

-->

## [Unreleased]

### Breaking changes

* Remove `TagKind::Clone` and handle as `Unknown` to fix issues with C# bindings

### Changed

* Bump upstream deps to ce2ba11a (check the [Upstream CHANGELOG] for more details)

### Added

* Add support for `i686-pc-windows-msvc` and `aarch64-pc-windows-msv`
* Add support to `i686-unknown-linux-gnu`
* Expose `Relay::ban`

<!-- Links -->
[Upstream CHANGELOG]: https://github.com/rust-nostr/nostr/blob/master/CHANGELOG.md
