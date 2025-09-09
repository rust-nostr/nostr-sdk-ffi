# Contributing

## Organization guidelines

This project follows the rust-nostr organization guidelines: https://github.com/rust-nostr/guidelines

## Additional repository guidelines

Currently `uniffi-bindgen-cs` don't support enum tuple-variant (https://github.com/NordSecurity/uniffi-bindgen-cs/issues/127),
so all enums variants that have data must be represented as struct-variants:

```rust
// BAD (not supported)
#[derive(Enum)]
pub enum MyEnum {
    Url(String),
}

// GOOD (supported)
#[derive(Enum)]
pub enum MyEnum {
    Url { url: String },
}
```
