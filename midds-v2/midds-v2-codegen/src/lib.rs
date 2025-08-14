//! # MIDDS V2 Codegen - Procedural Macro for Dual-Mode Type Generation
//!
//! This crate provides the `runtime_midds` procedural macro that enables automatic
//! transformation of Rust data structures between std and Substrate runtime modes.
//!
//! ## Overview
//!
//! The core functionality revolves around the `#[runtime_midds]` attribute macro that:
//! - Generates two versions of each annotated type (std and runtime)
//! - Automatically transforms `String` and `Vec<T>` fields to `BoundedVec` in runtime mode
//! - Adds appropriate trait derivations for each compilation mode
//! - Supports complex nested structures and enums
//!
//! ## Key Features
//!
//! ### Type Transformations
//! - `String` → `BoundedVec<u8, ConstU32<N>>`
//! - `Vec<T>` → `BoundedVec<T, ConstU32<N>>`
//! - `Option<String>` → `Option<BoundedVec<u8, ConstU32<N>>>`
//! - `Option<Vec<T>>` → `Option<BoundedVec<T, ConstU32<N>>>`
//! - Recursive transformation for nested `Option` types
//!
//! ### Bound Specification
//! Use `#[runtime_bound(N)]` attributes to specify maximum sizes:
//! - On struct fields for field-level bounds
//! - On enum variants for variant-level bounds (applies to all fields in that variant)
//!
//! ### Trait Derivations
//! - **Runtime mode**: `Encode`, `Decode`, `DecodeWithMemTracking`, `TypeInfo`, `MaxEncodedLen`, `Debug`, `Clone`, `PartialEq`, `Eq`
//! - **Std mode**: `Debug`, `Clone`, `PartialEq`, `Eq`
//!
//! ## Usage Examples
//!
//! ### Basic Struct
//! ```rust
//! use allfeat_midds_v2_codegen::runtime_midds;
//!
//! #[runtime_midds]
//! pub struct MyStruct {
//!     #[runtime_bound(256)]
//!     pub title: String,
//!
//!     #[runtime_bound(64)]
//!     pub tags: Vec<String>,
//!
//!     pub id: u64, // No transformation
//! }
//! ```
//!
//! ### Newtype Struct
//! ```rust
//! use allfeat_midds_v2_codegen::runtime_midds;
//!
//! #[runtime_midds]
//! pub struct Identifier(#[runtime_bound(32)] String);
//! ```
//!
//! ### Enum with Bounds
//! ```rust
//! use allfeat_midds_v2_codegen::runtime_midds;
//!
//! #[runtime_midds]
//! pub enum WorkType {
//!     Original,
//!     #[runtime_bound(512)]
//!     Medley(Vec<u64>),
//!     #[runtime_bound(256)]
//!     Adaptation(String, u32),
//! }
//! ```
//!
//! ### Optional Fields
//! ```rust
//! use allfeat_midds_v2_codegen::runtime_midds;
//!
//! #[runtime_midds]
//! pub struct OptionalData {
//!     #[runtime_bound(128)]
//!     pub optional_title: Option<String>,
//!
//!     #[runtime_bound(32)]
//!     pub optional_list: Option<Vec<u32>>,
//! }
//! ```
//!
//! ## Architecture
//!
//! The crate is organized into several modules:
//! - [`attribute`] - Parsing and validation of `#[runtime_bound(N)]` attributes
//! - [`transform`] - Type transformation logic between std and runtime modes
//! - [`generate`] - Code generation utilities for structs and enums
//! - [`error`] - Comprehensive error handling with detailed diagnostics

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

mod attribute;
mod error;
mod generate;
mod transform;

use attribute::AttributeParser;
use enum_handler::EnumHandler;
use error::{MacroError, MacroResult};
use generate::GenerationConfig;
use struct_handler::StructHandler;

mod enum_handler;
/// Sub-modules for handling different data structure types
mod struct_handler;

/// Attribute macro that transforms String and Vec<Type> fields to BoundedVec when runtime feature is enabled.
///
/// This is the core macro of the MIDDS V2 system, enabling dual-mode compilation of data structures
/// for both std Rust applications and Substrate blockchain runtime environments.
///
/// # Syntax
///
/// Apply the macro to structs and enums:
/// ```rust
/// use allfeat_midds_v2_codegen::runtime_midds;
///
/// #[runtime_midds]
/// pub struct MyType {
///     #[runtime_bound(256)]  // Specify bound for transformable fields
///     field: String,       // Will be transformed in runtime mode
///     other: u32,          // No transformation needed
/// }
/// ```
///
/// # Supported Types
///
/// ## Structs
/// - Named field structs: `struct S { field: Type }`
/// - Tuple structs: `struct S(Type, Type)`
/// - Unit structs: `struct S;`
///
/// ## Enums
/// - Unit variants: `Variant`
/// - Tuple variants: `Variant(Type, Type)`
/// - Struct variants: `Variant { field: Type }`
///
/// # Bounds
///
/// Use `#[runtime_bound(N)]` to specify maximum sizes:
///
/// ## Field-Level Bounds (Structs)
/// ```rust
/// # use allfeat_midds_v2_codegen::runtime_midds;
/// #[runtime_midds]
/// struct Example {
///     #[runtime_bound(256)]
///     title: String,
///     #[runtime_bound(64)]
///     tags: Vec<String>,
/// }
/// ```
///
/// ## Variant-Level Bounds (Enums)
/// ```rust
/// # use allfeat_midds_v2_codegen::runtime_midds;
/// #[runtime_midds]
/// enum Example {
///     Simple,
///     #[runtime_bound(128)]
///     WithData(String, Vec<u32>),
/// }
/// ```
///
/// # Transformations
///
/// | Original Type | Runtime Type |
/// |---------------|--------------|
/// | `String` | `BoundedVec<u8, ConstU32<N>>` |
/// | `Vec<T>` | `BoundedVec<T, ConstU32<N>>` |
/// | `Option<String>` | `Option<BoundedVec<u8, ConstU32<N>>>` |
/// | `Option<Vec<T>>` | `Option<BoundedVec<T, ConstU32<N>>>` |
/// | `&str` | `BoundedVec<u8, ConstU32<N>>` |
///
/// # Generated Traits
///
/// ## Runtime Mode (`#[cfg(feature = "runtime")]`)
/// - `parity_scale_codec::Encode`
/// - `parity_scale_codec::Decode`
/// - `parity_scale_codec::DecodeWithMemTracking`
/// - `scale_info::TypeInfo`
/// - `parity_scale_codec::MaxEncodedLen`
/// - `Debug`, `Clone`, `PartialEq`, `Eq`
///
/// ## Std Mode (`#[cfg(feature = "std")]`)
/// - `Debug`, `Clone`, `PartialEq`, `Eq`
///
/// # Examples
///
/// ## Complete Example
/// ```rust
/// use allfeat_midds_v2_codegen::runtime_midds;
///
/// #[runtime_midds]
/// pub struct MusicalWork {
///     #[runtime_bound(256)]
///     pub title: String,
///
///     #[runtime_bound(11)]
///     pub iswc: Option<String>,
///
///     #[runtime_bound(128)]
///     pub participants: Vec<u64>,
///
///     pub creation_year: Option<u16>,
///     pub bpm: Option<u16>,
/// }
/// ```
///
/// This generates two versions:
/// - Std: Uses `String`, `Vec<u64>`
/// - Runtime: Uses `BoundedVec<u8, ConstU32<256>>`, `BoundedVec<u64, ConstU32<128>>`
///
/// ## Error Handling
///
/// The macro will emit compile errors for:
/// - Missing `#[runtime_bound(N)]` on transformable fields
/// - Invalid bound syntax
/// - Unsupported type structures
///
/// # Implementation Notes
///
/// - Bounds are enforced at compile time in runtime mode
/// - The macro preserves all existing attributes except `#[runtime_bound]`
/// - Generic types are preserved and passed through unchanged
/// - The transformation is purely syntactic - no runtime overhead
#[proc_macro_attribute]
pub fn runtime_midds(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match process_derive_input(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

/// Main processing function for the derive input
fn process_derive_input(input: DeriveInput) -> MacroResult<proc_macro2::TokenStream> {
    // Create generation configuration
    let config = GenerationConfig::new(
        input.ident.clone(),
        input.vis.clone(),
        input.generics.clone(),
        AttributeParser::filter_runtime_bound_attrs(&input.attrs)
            .into_iter()
            .cloned()
            .collect(),
    );

    // Validate top-level attributes
    AttributeParser::validate_attributes(&input.attrs)?;

    // Process based on data structure type
    match input.data {
        Data::Struct(data_struct) => StructHandler::process_struct(&config, &data_struct),
        Data::Enum(data_enum) => EnumHandler::process_enum(&config, &data_enum),
        Data::Union(_) => Err(MacroError::unsupported_data_structure(
            &input,
            "union (only structs and enums are supported)",
        )),
    }
}
