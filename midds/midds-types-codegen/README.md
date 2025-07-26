# 🛠️ MIDDS Types Codegen

Procedural macros for generating bounded string and collection types with Substrate and WebAssembly compatibility.

## Overview

This crate provides macros to generate bounded types that work seamlessly with Substrate runtime and JavaScript bindings, solving `wasm_bindgen`'s generic parameter limitations.

## Macros

### `#[midds_string(bound)]`

Generates bounded string types with UTF-8 validation.

```rust
use midds_types_codegen::midds_string;

#[midds_string(256)]
pub struct TrackTitle;

// With regex validation
#[midds_string(15, regex = r"^[A-Z]{2}[A-Z0-9]{3}[0-9]{2}[0-9]{5}$")]
pub struct Isrc;

// Usage
let mut title = TrackTitle::from_str("My Song").unwrap();
title.push_str(" - Extended Mix").unwrap();
assert_eq!(title.as_str(), "My Song - Extended Mix");
```

### `#[midds_collection(type, bound)]`

Generates bounded collection types.

```rust
use midds_types_codegen::midds_collection;

#[midds_collection(u64, 64)]
pub struct ProducerIds;

// Usage
let mut producers = ProducerIds::new();
producers.push(12345).unwrap();
producers.push(67890).unwrap();
assert_eq!(producers.len(), 2);
```

## Generated Features

### Rust API
- Standard trait implementations (`Clone`, `PartialEq`, `Encode`, `Decode`, etc.)
- String/collection manipulation methods
- Type-specific error handling
- Substrate runtime compatibility

### JavaScript API (with `js` feature)
- `wasm_bindgen` bindings
- Property getters/setters
- Type-safe method exports
- Serde serialization support

## Key Benefits

- ✅ **Compile-time bounds**: Capacity limits enforced at compile time
- ✅ **Runtime validation**: UTF-8 and bounds checking at runtime
- ✅ **WASM compatibility**: JavaScript bindings without generic limitations
- ✅ **Memory efficient**: Wraps `sp_runtime::BoundedVec`
- ✅ **Type safety**: Strong typing with comprehensive error handling

## Error Handling

Each generated type includes a specific error enum:

```rust
// For strings
pub enum TrackTitleError {
    InvalidUtf8,
    TooLong,
    InvalidFormat, // If regex validation is used
}

// For collections  
pub enum ProducerIdsError {
    TooManyItems,
    InvalidItem,
}
```

## Dependencies

- [proc-macro2](https://docs.rs/proc-macro2) - Procedural macro utilities
- [quote](https://docs.rs/quote) - Code generation
- [syn](https://docs.rs/syn) - Rust syntax parsing