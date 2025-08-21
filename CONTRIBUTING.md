# Contributing

Currently `uniffi-bindgen-cs` and `gobley` don't support enum tuple-variant 
(https://github.com/NordSecurity/uniffi-bindgen-cs/issues/127 and https://github.com/gobley/gobley/issues/193),
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
