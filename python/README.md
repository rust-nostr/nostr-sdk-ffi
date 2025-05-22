# Nostr SDK

## Description

Nostr protocol implementation, Relay, RelayPool, high-level client library, NWC client and more.

## Documentation

Learn more about `rust-nostr` at <https://rust-nostr.org>.

## Supported NIPs

Look at <https://github.com/rust-nostr/nostr/tree/master/crates/nostr#supported-nips>

### Supported platforms

The following OS and architectures are supported:

| OS            | x86_64 | aarch64 | armv7 | i686 | riscv64 |
|---------------|--------|---------|-------|------|---------|
| Android       | ❌      | ❌       | ❌     | ❌    | ❌       |
| Linux (GLIBC) | ✅      | ✅       | ✅     | ✅    | ✅*      |
| Linux (MUSL)  | ✅      | ✅       | ✅     | ✅    | ✅*      |
| macOS         | ✅      | ✅       | ❌     | ❌    | ❌       |
| Windows       | ✅      | ✅       | ❌     | ✅    | ❌       |

Are you interested in other platforms? Open an issue [here](https://github.com/rust-nostr/nostr-sdk-ffi).

<small>* PyPI currently doesn't allow uploading riscv64 wheels</small>

## State

**This library is in an ALPHA state**, things that are implemented generally work but the API will change in breaking ways.

## Donations

`rust-nostr` is free and open-source. This means we do not earn any revenue by selling it. Instead, we rely on your financial support. If you actively use any of the `rust-nostr` libs/software/services, then please [donate](https://rust-nostr.org/donate).

## License

This project is distributed under the MIT software license - see the [LICENSE](https://rust-nostr.org/license) file for details
