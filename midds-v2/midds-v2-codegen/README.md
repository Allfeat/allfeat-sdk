# MIDDS V2 Codegen - Procedural Macro for Dual-Mode Types

A Rust procedural macro crate that enables automatic transformation of data structures between native Rust types and Substrate runtime types.

## Overview

The `runtime_midds` macro is the core of the MIDDS V2 dual-mode compilation system. It automatically generates two versions of your data structures:

- **Native Mode**: Uses standard Rust types (`String`, `Vec<T>`)
- **Runtime Mode**: Uses Substrate-compatible types (`BoundedVec`)

## Key Features

✨ **Automatic Type Transformation**
- `String` → `BoundedVec<u8, ConstU32<N>>`
- `Vec<T>` → `BoundedVec<T, ConstU32<N>>`
- Recursive transformation for `Option<T>` wrappers

🏗️ **Flexible Structure Support**
- Named field structs
- Tuple structs (newtypes)
- Unit structs
- Enums with all variant types

🔧 **Configurable Bounds**
- Field-level bounds with `#[runtime_bound(N)]`
- Variant-level bounds for enums
- Compile-time validation

🚀 **Automatic Trait Derivation**
- Substrate traits in runtime mode
- Standard traits in both modes

## Quick Start

### Basic Usage

```rust
use allfeat_midds_v2_codegen::runtime_midds;

#[runtime_midds]
pub struct MyData {
    #[runtime_bound(256)]
    pub title: String,
    
    #[runtime_bound(64)]
    pub tags: Vec<String>,
    
    pub id: u64, // No transformation
}
```

This generates:

**Native Mode** (`cargo build --features="native"`):
```rust
pub struct MyData {
    pub title: String,
    pub tags: Vec<String>,
    pub id: u64,
}
```

**Runtime Mode** (`cargo build --features="runtime"`):
```rust  
pub struct MyData {
    pub title: BoundedVec<u8, ConstU32<256>>,
    pub tags: BoundedVec<BoundedVec<u8, ConstU32<256>>, ConstU32<64>>,
    pub id: u64,
}
```

### Newtype Structs

```rust
#[runtime_midds]
pub struct Identifier(#[runtime_bound(32)] String);

#[runtime_midds] 
pub struct TrackList(#[runtime_bound(100)] Vec<u64>);
```

### Enums

```rust
#[runtime_midds]
pub enum WorkType {
    Original,
    #[runtime_bound(512)]
    Medley(Vec<u64>),    // All fields use this bound
    #[runtime_bound(256)]
    Remix(String, u32),  // Both fields use this bound
    Adaptation(u64),     // No transformation needed
}
```

### Optional Fields

```rust
#[runtime_midds]
pub struct OptionalData {
    #[runtime_bound(128)]
    pub maybe_title: Option<String>,
    
    #[runtime_bound(32)]
    pub maybe_list: Option<Vec<u32>>,
    
    #[runtime_bound(64)]
    pub nested: Option<Option<String>>, // Recursive transformation
}
```

## Transformation Reference

| Original Type | Runtime Transformation |
|---------------|------------------------|
| `String` | `BoundedVec<u8, ConstU32<N>>` |
| `Vec<T>` | `BoundedVec<T, ConstU32<N>>` |
| `Option<String>` | `Option<BoundedVec<u8, ConstU32<N>>>` |
| `Option<Vec<T>>` | `Option<BoundedVec<T, ConstU32<N>>>` |
| `&str` | `BoundedVec<u8, ConstU32<N>>` |
| Other types | No transformation |

## Bound Specification

### Field-Level Bounds (Structs)
```rust
#[runtime_midds]
struct Example {
    #[runtime_bound(256)]  // This field only
    title: String,
    
    #[runtime_bound(64)]   // This field only
    tags: Vec<String>,
    
    id: u64,  // No bound needed
}
```

### Variant-Level Bounds (Enums)
```rust
#[runtime_midds]
enum Example {
    Simple,
    
    #[runtime_bound(128)]  // Applies to ALL fields in this variant
    Complex(String, Vec<u32>, Option<String>),
}
```

## Generated Traits

### Runtime Mode
When `runtime` feature is enabled:
```rust
#[derive(
    parity_scale_codec::Encode,
    parity_scale_codec::Decode,
    parity_scale_codec::DecodeWithMemTracking,
    scale_info::TypeInfo,
    parity_scale_codec::MaxEncodedLen,
    Debug,
    Clone,
    PartialEq,
    Eq
)]
```

### Native Mode  
When `runtime` feature is disabled:
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
```

## Error Handling

The macro provides helpful compile-time errors:

### Missing Bounds
```rust
#[runtime_midds]
struct Example {
    title: String, // ❌ Error: Missing #[runtime_bound(N)]
}
```
**Error**: `String` fields require `#[runtime_bound(N)]` attribute.

### Invalid Bound Syntax
```rust
#[runtime_midds]
struct Example {
    #[runtime_bound("invalid")]  // ❌ Error: Must be integer
    title: String,
}
```

### Unsupported Patterns
```rust
#[runtime_midds]
union MyUnion { // ❌ Error: Unions not supported
    a: u32,
    b: f32,
}
```

## Best Practices

### 1. Choose Appropriate Bounds
```rust
#[runtime_bound(32)]   // Short identifiers (ISWC, ISRC)
#[runtime_bound(256)]  // Titles, names, descriptions  
#[runtime_bound(1024)] // Large collections (track lists)
```

### 2. Use Meaningful Names
```rust
#[runtime_midds]
pub struct Track {
    #[runtime_bound(12)]  // ISRC length
    pub isrc: String,
    
    #[runtime_bound(256)] // Reasonable title length
    pub title: String,
}
```

### 3. Document Your Bounds
```rust
/// Musical work with industry-standard bounds
#[runtime_midds]
pub struct MusicalWork {
    /// ISWC code (11 characters max)
    #[runtime_bound(11)]
    pub iswc: String,
    
    /// Work title (256 characters max) 
    #[runtime_bound(256)]
    pub title: String,
}
```

## Advanced Usage

### Complex Nested Structures
```rust
#[runtime_midds]
pub struct ComplexType {
    #[runtime_bound(100)]
    pub nested_options: Option<Vec<Option<String>>>,
    
    #[runtime_bound(50)]
    pub deep_nesting: Option<Option<Vec<u64>>>,
}
```

### Integration with Other Macros
```rust
#[runtime_midds]
#[serde(rename_all = "camelCase")] // Other attributes preserved
pub struct SerializableData {
    #[runtime_bound(256)]
    #[serde(rename = "trackTitle")]
    pub title: String,
}
```

### Generic Types
```rust
#[runtime_midds]
pub struct GenericContainer<T> {
    pub data: T,           // Generic types passed through
    
    #[runtime_bound(64)]
    pub metadata: Vec<String>, // Still transformed
}
```

## Implementation Details

### Conditional Compilation
The macro generates feature-gated code blocks:

```rust
#[cfg(feature = "runtime")]
// Runtime version with BoundedVec types

#[cfg(not(feature = "runtime"))]  
// Native version with standard types
```

### Attribute Filtering
- `#[runtime_bound(N)]` attributes are removed from final output
- All other attributes are preserved
- Derives are added automatically based on compilation mode

### Type Analysis
The macro performs deep analysis of type structures:
- Recursively processes `Option<T>` wrappers
- Identifies transformable types (`String`, `Vec<T>`, `&str`)
- Preserves complex generic parameters

## Contributing

When contributing to the macro:

1. **Add Tests**: Include test cases for new functionality
2. **Document Changes**: Update both code docs and README
3. **Handle Errors**: Provide clear error messages for edge cases
4. **Maintain Compatibility**: Ensure existing code continues to work

## License

This crate is part of the Allfeat SDK and is licensed under the GNU General Public License v3.0.