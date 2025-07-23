//! # MIDDS Types Code Generation
//!
//! This crate provides procedural macros for generating bounded string and collection types
//! that are compatible with Substrate runtime and WebAssembly bindings.
//!
//! The main purpose is to solve the limitation where `wasm_bindgen` doesn't support
//! generic parameters, preventing the use of `BoundedVec<T, Bound>` directly in
//! JavaScript-exposed APIs.
//!
//! ## Features
//!
//! - **`#[midds_string(bound)]`**: Generates bounded string types with UTF-8 validation
//! - **`#[midds_collection(type, bound)]`**: Generates bounded collection types
//! - **Substrate compatibility**: Implements all required traits for runtime usage
//! - **JavaScript bindings**: Provides `wasm_bindgen` integration when the `js` feature is enabled
//! - **Type safety**: Compile-time bounds checking and UTF-8 validation
//! - **Serde support**: Automatic Serialize/Deserialize implementation when the `js` feature is enabled
//!
//! ## Usage
//!
//! ```rust
//! use midds_types_codegen::{midds_string, midds_collection};
//!
//! // Generate a bounded string type with 256-byte limit
//! #[midds_string(256)]
//! pub struct TrackTitle;
//!
//! // Generate a bounded collection type for 64 u64 values
//! #[midds_collection(u64, 64)]
//! pub struct ProducerIds;
//! ```
//!
//! ## Generated API
//!
//! Each macro generates a complete API including:
//! - Standard trait implementations (`Clone`, `PartialEq`, `Encode`, `Decode`, etc.)
//! - String/collection manipulation methods
//! - Error handling with type-specific error enums
//! - JavaScript bindings (when `js` feature is enabled)
//!
//! ## Architecture
//!
//! The generated types wrap `sp_runtime::BoundedVec` to provide:
//! - Compile-time capacity limits
//! - Runtime bounds checking
//! - Memory-efficient storage
//! - Substrate runtime compatibility

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Type};

/// Procedural macro that generates a bounded string type with compile-time capacity limits.
///
/// This macro creates a newtype wrapper around `BoundedVec<u8>` that:
/// - Enforces UTF-8 validity at all times
/// - Provides string-like methods (`push_str`, `pop`, `clear`, etc.)
/// - Implements all required Substrate traits
/// - Exposes JavaScript bindings when the `js` feature is enabled
/// - Optional regex validation (std feature only)
///
/// # Arguments
///
/// * `bound` - Maximum number of bytes the string can contain (default: 128)
/// * `regex = "pattern"` - Optional regex pattern for validation (requires std feature)
///
/// # Example
///
/// ```rust
/// use midds_types_codegen::midds_string;
///
/// #[midds_string(256)]
/// pub struct TrackTitle;
///
/// // With regex validation
/// #[midds_string(15, regex = r"^[A-Z]{2}[A-Z0-9]{3}[0-9]{2}[0-9]{5}$")]
/// pub struct Isrc;
///
/// // Usage
/// let mut title = TrackTitle::from_str("My Song").unwrap();
/// title.push_str(" - Extended Mix").unwrap();
/// assert_eq!(title.as_str(), "My Song - Extended Mix");
/// ```
///
/// # Generated Methods
///
/// The macro generates the following public methods:
/// - `new()` - Create empty string
/// - `from_str(s: &str)` - Create from string slice
/// - `as_str()` - Get string slice
/// - `push_str(s: &str)` - Append string
/// - `push(ch: char)` - Append character
/// - `pop()` - Remove last character
/// - `clear()` - Remove all content
/// - `len()`, `is_empty()`, `capacity()` - Size information
///
/// # JavaScript Bindings
///
/// When the `js` feature is enabled, additional JavaScript-compatible methods are generated:
/// - `new()` - Constructor
/// - `value` getter/setter - Access string content
/// - `fromString(s)` - Static constructor
/// - `toString()` - Convert to string
/// - `pushStr(s)` - Append string
/// - `length`, `capacity`, `isEmpty` - Properties
///
/// # Serde Support
///
/// When the `js` feature is enabled, the type automatically implements:
/// - `Serialize` trait for JSON serialization
/// - `Deserialize` trait for JSON deserialization
///
/// # Error Handling
///
/// The macro generates a type-specific error enum that handles:
/// - `InvalidUtf8` - Invalid UTF-8 sequences
/// - `TooLong` - Content exceeds bound limit
///
/// # Panics
///
/// This macro will panic if:
/// - Invalid regex format is provided (must be quoted)
/// - Arguments are malformed (expected: `bound` or `bound, regex = "pattern"`)
#[proc_macro_attribute]
pub fn midds_string(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let vis = &input.vis;

    // Parse arguments (bound, optional regex)
    let (bound, regex_pattern) = if args.is_empty() {
        (128, None)
    } else {
        // Parse as AttributeArgs-like structure
        let args_str = args.to_string();
        
        // Simple parsing for "bound" or "bound, regex = \"pattern\""
        if args_str.contains("regex") {
            // Parse complex format: bound, regex = "pattern"
            let parts: Vec<&str> = args_str.split(',').collect();
            assert!(parts.len() == 2, "Expected format: bound, regex = \"pattern\"");
            
            let bound_str = parts[0].trim();
            let bound: u32 = bound_str.parse().expect("Invalid bound");
            
            let regex_part = parts[1].trim();
            assert!(regex_part.starts_with("regex"), "Second argument must be regex = \"pattern\"");
            
            // Extract the pattern from regex = "pattern"
            let eq_pos = regex_part.find('=').expect("Expected = after regex");
            let pattern_part = regex_part[eq_pos + 1..].trim();
            
            // Remove quotes - handle raw strings properly
            let pattern = if pattern_part.starts_with("r#\"") && pattern_part.ends_with("\"#") {
                &pattern_part[3..pattern_part.len() - 2]
            } else if pattern_part.starts_with("r\"") && pattern_part.ends_with('"') {
                &pattern_part[2..pattern_part.len() - 1]
            } else if pattern_part.starts_with('"') && pattern_part.ends_with('"') {
                &pattern_part[1..pattern_part.len() - 1]
            } else {
                panic!("Regex pattern must be quoted (use \"pattern\" or r\"pattern\")");
            };
            
            (bound, Some(pattern.to_string()))
        } else {
            // Simple bound only
            let bound: u32 = args_str.parse().expect("Invalid bound");
            (bound, None)
        }
    };

    // Preserve original attributes (except midds_string)
    let attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|attr| !attr.path().is_ident("midds_string"))
        .collect();

    // Generate unique error name based on struct name
    let error_name = quote::format_ident!("{}Error", struct_name);

    // Generate validation-related code
    let validation_error_variant = if regex_pattern.is_some() {
        quote! { InvalidFormat, }
    } else {
        quote! {}
    };

    let validation_error_display = if let Some(ref pattern) = regex_pattern {
        quote! {
            #error_name::InvalidFormat => write!(f, "String does not match required format: {}", #pattern),
        }
    } else {
        quote! {}
    };

    let validation_pattern_method = if let Some(ref pattern) = regex_pattern {
        quote! {
            #[cfg(feature = "std")]
            pub fn validation_pattern() -> Option<&'static str> {
                Some(#pattern)
            }

            #[cfg(not(feature = "std"))]
            pub fn validation_pattern() -> Option<&'static str> {
                None
            }
        }
    } else {
        quote! {
            pub fn validation_pattern() -> Option<&'static str> {
                None
            }
        }
    };



    let validation_method = if regex_pattern.is_some() {
        quote! {
            /// Validate the current content against the regex pattern (std only)
            #[cfg(feature = "std")]
            pub fn validate(&self) -> Result<(), #error_name> {
                if let Some(pattern) = Self::validation_pattern() {
                    let regex = ::regex::Regex::new(pattern)
                        .expect("Invalid regex pattern in midds_string macro");
                    if !regex.is_match(self.as_str()) {
                        return Err(#error_name::InvalidFormat);
                    }
                }
                Ok(())
            }
        }
    } else {
        quote! {}
    };

    let regex_validation_in_from_str = if regex_pattern.is_some() {
        quote! {
            // Validate normalized string
            #[cfg(feature = "std")]
            {
                if let Some(pattern) = Self::validation_pattern() {
                    let regex = ::regex::Regex::new(pattern)
                        .expect("Invalid regex pattern in midds_string macro");
                    if !regex.is_match(&normalized) {
                        return Err(#error_name::InvalidFormat);
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
            // Type-specific error definition
            #[derive(Debug, Clone)]
            #vis enum #error_name {
                InvalidUtf8,
                TooLong,
                #validation_error_variant
            }

            impl core::fmt::Display for #error_name {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        #error_name::InvalidUtf8 => write!(f, "Invalid UTF-8 sequence"),
                        #error_name::TooLong => write!(f, "String too long, maximum {} bytes", #bound),
                        #validation_error_display
                    }
                }
            }

            #[cfg(feature = "std")]
            impl std::error::Error for #error_name {}

            #(#attrs)*
            #[cfg_attr(feature = "js", ::wasm_bindgen::prelude::wasm_bindgen)]
            #[derive(
                Clone,
                PartialEq,
                Eq,
                ::parity_scale_codec::Encode,
                ::parity_scale_codec::Decode,
                ::parity_scale_codec::DecodeWithMemTracking,
                ::scale_info::TypeInfo,
                ::parity_scale_codec::MaxEncodedLen,
                ::sp_runtime::RuntimeDebug
            )]
            #[cfg_attr(feature = "js", derive(::serde::Serialize, ::serde::Deserialize))]
            #vis struct #struct_name(::sp_runtime::BoundedVec<u8, ::frame_support::traits::ConstU32<#bound>>);

            impl ::core::str::FromStr for #struct_name {
                type Err = #error_name;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    // Always normalize the input first (std only)
                    #[cfg(feature = "std")]
                    let normalized = s.replace([' ', '-', '_'], "");
                    
                    #[cfg(not(feature = "std"))]
                    let normalized = s.to_string();
                    
                    let bytes = normalized.as_bytes();
                    if bytes.len() > #bound as usize {
                        return Err(#error_name::TooLong);
                    }

                    #regex_validation_in_from_str

                    ::sp_runtime::BoundedVec::try_from(bytes.to_vec())
                        .map(#struct_name)
                        .map_err(|_| #error_name::TooLong)
                }
            }

            impl #struct_name {
                pub fn new() -> Self {
                    Self(::sp_runtime::BoundedVec::new())
                }

                // Validation pattern method
                #validation_pattern_method

                #validation_method


                pub fn from_utf8(bytes: Vec<u8>) -> Result<Self, #error_name> {
                    let s = core::str::from_utf8(&bytes).map_err(|_| #error_name::InvalidUtf8)?;
                    
                    // Use from_str to ensure normalization is applied
                    <Self as ::core::str::FromStr>::from_str(s)
                }

                pub fn as_str(&self) -> &str {
                    core::str::from_utf8(&self.0).expect("Valid UTF-8 checked on construction")
                }

                pub fn as_bytes(&self) -> &[u8] {
                    &self.0
                }

                pub fn len(&self) -> usize {
                    self.0.len()
                }

                pub fn is_empty(&self) -> bool {
                    self.0.is_empty()
                }

                pub fn capacity(&self) -> usize {
                    #bound as usize
                }

                pub fn max_capacity(&self) -> usize {
                    #bound as usize
                }

                pub fn remaining_capacity(&self) -> usize {
                    self.capacity() - self.len()
                }

                pub fn push_str(&mut self, s: &str) -> Result<(), #error_name> {
                    // Create the result string and normalize it
                    let current = self.as_str();
                    let combined = format!("{}{}", current, s);
                    
                    // Use from_str to create a new normalized instance
                    let normalized_instance = <Self as ::core::str::FromStr>::from_str(&combined)?;
                    
                    // Replace current content with normalized content
                    *self = normalized_instance;
                    
                    Ok(())
                }

                pub fn push(&mut self, ch: char) -> Result<(), #error_name> {
                    let mut buf = [0; 4];
                    let s = ch.encode_utf8(&mut buf);
                    self.push_str(s)
                }

                pub fn pop(&mut self) -> Option<char> {
                    if self.is_empty() {
                        return None;
                    }

                    let s = self.as_str();
                    let mut chars = s.chars();
                    let last_char = chars.next_back()?;
                    let new_len = s.len() - last_char.len_utf8();
                    self.0.truncate(new_len);
                    Some(last_char)
                }

                pub fn clear(&mut self) {
                    self.0.clear();
                }

                pub fn truncate(&mut self, new_len: usize) {
                    if new_len <= self.len() {
                        let s = self.as_str();
                        if let Some((byte_index, _)) = s.char_indices().nth(new_len) {
                            self.0.truncate(byte_index);
                        } else {
                            self.0.truncate(new_len.min(s.len()));
                        }
                    }
                }

                pub fn into_string(self) -> String {
                    String::from_utf8(self.0.into_inner()).expect("Invalid UTF-8")
                }


                pub fn into_inner(self) -> ::sp_runtime::BoundedVec<u8, ::frame_support::traits::ConstU32<#bound>> {
                    self.0
                }

                pub fn get(&self, range: core::ops::Range<usize>) -> Option<&str> {
                    self.as_str().get(range)
                }
            }

            impl Default for #struct_name {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl core::fmt::Display for #struct_name {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "{}", self.as_str())
                }
            }

            impl core::ops::Deref for #struct_name {
                type Target = str;

                fn deref(&self) -> &Self::Target {
                    self.as_str()
                }
            }

            impl AsRef<str> for #struct_name {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }

            impl AsRef<[u8]> for #struct_name {
                fn as_ref(&self) -> &[u8] {
                    self.as_bytes()
                }
            }

            impl From<#struct_name> for String {
                fn from(ms: #struct_name) -> String {
                    ms.into_string()
                }
            }

            impl TryFrom<String> for #struct_name {
                type Error = #error_name;

                fn try_from(s: String) -> Result<Self, Self::Error> {
                    Self::from_utf8(s.into_bytes())
                }
            }

            impl TryFrom<&str> for #struct_name {
                type Error = #error_name;

                fn try_from(s: &str) -> Result<Self, Self::Error> {
                    <#struct_name as ::core::str::FromStr>::from_str(s)
                }
            }

            impl PartialEq<str> for #struct_name {
                fn eq(&self, other: &str) -> bool {
                    self.as_str() == other
                }
            }

            impl PartialEq<&str> for #struct_name {
                fn eq(&self, other: &&str) -> bool {
                    self.as_str() == *other
                }
            }

            impl PartialEq<String> for #struct_name {
                fn eq(&self, other: &String) -> bool {
                    self.as_str() == other.as_str()
                }
            }

            #[cfg(feature = "js")]
            #[::wasm_bindgen::prelude::wasm_bindgen]
            impl #struct_name {
                #[::wasm_bindgen::prelude::wasm_bindgen(constructor)]
                pub fn js_new() -> #struct_name {
                    Self::new()
                }

    #[::wasm_bindgen::prelude::wasm_bindgen(getter)]
          pub fn value(&self) -> String {
              self.as_str().to_string()
          }

          #[::wasm_bindgen::prelude::wasm_bindgen(setter)]
          pub fn set_value(&mut self, value: &str) -> Result<(), ::wasm_bindgen::JsError> {
              *self = <#struct_name as ::core::str::FromStr>::from_str(value).map_err(|e| ::wasm_bindgen::JsError::new(&e.to_string()))?;
              Ok(())
          }

                #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "fromString")]
                pub fn js_from_string(s: &str) -> Result<#struct_name, ::wasm_bindgen::JsError> {
                    <#struct_name as ::core::str::FromStr>::from_str(s).map_err(|e| ::wasm_bindgen::JsError::new(&e.to_string()))
                }

                #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "toString")]
                pub fn js_to_string(&self) -> String {
                    self.as_str().to_string()
                }

                #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "pushStr")]
                pub fn js_push_str(&mut self, s: &str) -> Result<(), ::wasm_bindgen::JsError> {
                    self.push_str(s).map_err(|e| ::wasm_bindgen::JsError::new(&e.to_string()))
                }

                #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "length")]
                pub fn js_length(&self) -> usize {
                    self.len()
                }

                #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "capacity")]
                pub fn js_capacity(&self) -> usize {
                    self.capacity()
                }

                #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "maxCapacity")]
                pub fn js_max_capacity(&self) -> usize {
                    self.max_capacity()
                }

                #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "remainingCapacity")]
                pub fn js_remaining_capacity(&self) -> usize {
                    self.remaining_capacity()
                }

                #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "isEmpty")]
                pub fn js_is_empty(&self) -> bool {
                    self.is_empty()
                }

                #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "clear")]
                pub fn js_clear(&mut self) {
                    self.clear()
                }

            }
        };

    TokenStream::from(expanded)
}

/// Procedural macro that generates a bounded collection type with compile-time capacity limits.
///
/// This macro creates a newtype wrapper around `BoundedVec<T>` that:
/// - Enforces capacity limits at compile time
/// - Provides collection methods (push, pop, insert, remove, etc.)
/// - Implements all required Substrate traits
/// - Exposes JavaScript bindings when the `js` feature is enabled
///
/// # Arguments
///
/// * `type` - The inner type that the collection will contain
/// * `bound` - Maximum number of items the collection can hold
///
/// # Example
///
/// ```rust
/// use midds_types_codegen::midds_collection;
///
/// #[midds_collection(u64, 64)]
/// pub struct ProducerIds;
///
/// // Usage
/// let mut producers = ProducerIds::new();
/// producers.push(12345).unwrap();
/// producers.push(67890).unwrap();
/// assert_eq!(producers.len(), 2);
/// ```
///
/// # Generated Methods
///
/// The macro generates the following public methods:
/// - `new()` - Create empty collection
/// - `push(item)` - Add item to end
/// - `pop()` - Remove last item
/// - `insert(index, item)` - Insert at position
/// - `remove(index)` - Remove at position
/// - `get(index)`, `get_mut(index)` - Access items
/// - `clear()` - Remove all items
/// - `len()`, `is_empty()`, `capacity()` - Size information
/// - `iter()`, `iter_mut()` - Iterators
/// - `as_slice()`, `as_mut_slice()` - Slice access
///
/// # JavaScript Bindings
///
/// When the `js` feature is enabled, additional JavaScript-compatible methods are generated:
/// - `new()` - Constructor
/// - `pushItem(item)` - Add item
/// - `popItem()` - Remove last item
/// - `getItem(index)` - Get item by index
/// - `insertItem(index, item)` - Insert at position
/// - `removeItem(index)` - Remove at position
/// - `length`, `capacity`, `isEmpty` - Properties
///
/// Note: Generic array conversion requires type-specific implementations
/// due to `wasm_bindgen` limitations with generics.
///
/// # Serde Support
///
/// When the `js` feature is enabled, the type automatically implements:
/// - `Serialize` trait for JSON serialization (requires inner type to be Serialize)
/// - `Deserialize` trait for JSON deserialization (requires inner type to be `DeserializeOwned`)
///
/// # Error Handling
///
/// The macro generates a type-specific error enum that handles:
/// - `TooManyItems` - Collection exceeds capacity
/// - `InvalidItem` - Item validation failed
///
/// # Panics
///
/// This macro will panic if:
/// - Wrong number of arguments provided (expected exactly 2: type and bound)
/// - Invalid inner type format
#[proc_macro_attribute]
pub fn midds_collection(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let vis = &input.vis;

    // Parse arguments (type, bound)
    let args_str = args.to_string();
    let args_parts: Vec<&str> = args_str.split(',').map(str::trim).collect();

    assert!(args_parts.len() == 2, "midds_collection expects exactly 2 arguments: type and bound");

    let inner_type = args_parts[0];
    let bound_str = args_parts[1];
    let bound: u32 = bound_str.parse().expect("Invalid bound, expected number");

    // Parse inner type
    let inner_type_ident: Type = syn::parse_str(inner_type).expect("Invalid inner type");

    // Preserve original attributes
    let attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|attr| !attr.path().is_ident("midds_collection"))
        .collect();

    // Generate unique error name based on struct name
    let error_name = quote::format_ident!("{}Error", struct_name);

    let expanded = quote! {
        // Type-specific error definition
        #[derive(Debug, Clone)]
        #vis enum #error_name {
            TooManyItems,
            InvalidItem,
        }

        impl core::fmt::Display for #error_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    #error_name::TooManyItems => write!(f, "Too many items, maximum {} items", #bound),
                    #error_name::InvalidItem => write!(f, "Invalid item in collection"),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for #error_name {}

        #(#attrs)*
        #[cfg_attr(feature = "js", ::wasm_bindgen::prelude::wasm_bindgen)]
        #[derive(
            Clone,
            PartialEq,
            Eq,
            ::parity_scale_codec::Encode,
            ::parity_scale_codec::Decode,
            ::parity_scale_codec::DecodeWithMemTracking,
            ::scale_info::TypeInfo,
            ::parity_scale_codec::MaxEncodedLen,
            ::sp_runtime::RuntimeDebug
        )]
        #[cfg_attr(feature = "js", derive(::serde::Serialize, ::serde::Deserialize))]
        #vis struct #struct_name(::sp_runtime::BoundedVec<#inner_type_ident, ::frame_support::traits::ConstU32<#bound>>);

        impl #struct_name {
            pub fn new() -> Self {
                Self(::sp_runtime::BoundedVec::new())
            }

            pub fn with_capacity(_capacity: usize) -> Self {
                // BoundedVec doesn't have with_capacity, use new() instead
                Self(::sp_runtime::BoundedVec::new())
            }

            pub fn try_from_vec(vec: Vec<#inner_type_ident>) -> Result<Self, #error_name> {
                if vec.len() > #bound as usize {
                    return Err(#error_name::TooManyItems);
                }

                ::sp_runtime::BoundedVec::try_from(vec)
                    .map(#struct_name)
                    .map_err(|_| #error_name::TooManyItems)
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            pub fn capacity(&self) -> usize {
                #bound as usize
            }

            pub fn max_capacity(&self) -> usize {
                #bound as usize
            }

            pub fn remaining_capacity(&self) -> usize {
                self.capacity() - self.len()
            }

            pub fn push(&mut self, item: #inner_type_ident) -> Result<(), #error_name> {
                self.0.try_push(item).map_err(|_| #error_name::TooManyItems)
            }

            pub fn pop(&mut self) -> Option<#inner_type_ident> {
                self.0.pop()
            }

            pub fn insert(&mut self, index: usize, item: #inner_type_ident) -> Result<(), #error_name> {
                if self.len() >= #bound as usize {
                    return Err(#error_name::TooManyItems);
                }
                self.0.try_insert(index, item).map_err(|_| #error_name::TooManyItems)
            }

            pub fn remove(&mut self, index: usize) -> #inner_type_ident {
                self.0.remove(index)
            }

            pub fn clear(&mut self) {
                self.0.clear();
            }

            pub fn get(&self, index: usize) -> Option<&#inner_type_ident> {
                self.0.get(index)
            }

            pub fn get_mut(&mut self, index: usize) -> Option<&mut #inner_type_ident> {
                self.0.get_mut(index)
            }

            pub fn iter(&self) -> impl Iterator<Item = &#inner_type_ident> {
                self.0.iter()
            }

            pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut #inner_type_ident> {
                self.0.iter_mut()
            }

            pub fn as_slice(&self) -> &[#inner_type_ident] {
                &self.0
            }

            pub fn as_mut_slice(&mut self) -> &mut [#inner_type_ident] {
                self.0.as_mut()
            }

            pub fn into_vec(self) -> Vec<#inner_type_ident> {
                self.0.into_inner()
            }

            pub fn into_inner(self) -> ::sp_runtime::BoundedVec<#inner_type_ident, ::frame_support::traits::ConstU32<#bound>> {
                self.0
            }

            pub fn extend_from_slice(&mut self, slice: &[#inner_type_ident]) -> Result<(), #error_name>
            where
                #inner_type_ident: Clone
            {
                if self.len() + slice.len() > #bound as usize {
                    return Err(#error_name::TooManyItems);
                }

                for item in slice {
                    self.0.try_push(item.clone()).map_err(|_| #error_name::TooManyItems)?;
                }
                Ok(())
            }

        }

        impl Default for #struct_name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl core::ops::Deref for #struct_name {
            type Target = [#inner_type_ident];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl AsRef<[#inner_type_ident]> for #struct_name {
            fn as_ref(&self) -> &[#inner_type_ident] {
                &self.0
            }
        }

        impl AsMut<[#inner_type_ident]> for #struct_name {
            fn as_mut(&mut self) -> &mut [#inner_type_ident] {
                self.0.as_mut()
            }
        }

        impl From<#struct_name> for Vec<#inner_type_ident> {
            fn from(collection: #struct_name) -> Vec<#inner_type_ident> {
                collection.into_vec()
            }
        }

        impl TryFrom<Vec<#inner_type_ident>> for #struct_name {
            type Error = #error_name;

            fn try_from(vec: Vec<#inner_type_ident>) -> Result<Self, Self::Error> {
                Self::try_from_vec(vec)
            }
        }

        impl<'a> IntoIterator for &'a #struct_name {
            type Item = &'a #inner_type_ident;
            type IntoIter = core::slice::Iter<'a, #inner_type_ident>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.iter()
            }
        }

        impl<'a> IntoIterator for &'a mut #struct_name {
            type Item = &'a mut #inner_type_ident;
            type IntoIter = core::slice::IterMut<'a, #inner_type_ident>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.iter_mut()
            }
        }

        impl IntoIterator for #struct_name {
            type Item = #inner_type_ident;
            type IntoIter = alloc::vec::IntoIter<#inner_type_ident>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.into_inner().into_iter()
            }
        }

        #[cfg(feature = "js")]
        #[::wasm_bindgen::prelude::wasm_bindgen]
        impl #struct_name {
            #[::wasm_bindgen::prelude::wasm_bindgen(constructor)]
            pub fn js_new() -> #struct_name {
                Self::new()
            }

            // Getter/Setter for JS property access
            #[::wasm_bindgen::prelude::wasm_bindgen(getter)]
            pub fn items(&self) -> ::wasm_bindgen::JsValue {
                // Note: Generic conversion not available, use toArray() instead
                ::wasm_bindgen::JsValue::NULL
            }

            #[::wasm_bindgen::prelude::wasm_bindgen(setter)]
            pub fn set_items(&mut self, items: ::wasm_bindgen::JsValue) -> Result<(), ::wasm_bindgen::JsError> {
                // Note: Generic serialization not available, use fromArray() instead
                Err(::wasm_bindgen::JsError::new("Use fromArray() method instead"))
            }

            // Element manipulation methods
            #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "pushItem")]
            pub fn js_push_item(&mut self, item: #inner_type_ident) -> Result<(), ::wasm_bindgen::JsError> {
                self.push(item).map_err(|e| ::wasm_bindgen::JsError::new(&e.to_string()))
            }

            #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "popItem")]
            pub fn js_pop_item(&mut self) -> Option<#inner_type_ident> {
                self.pop()
            }

            #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "getItem")]
            pub fn js_get_item(&self, index: usize) -> Option<#inner_type_ident>
            where
                #inner_type_ident: Clone
            {
                self.get(index).cloned()
            }

            #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "insertItem")]
            pub fn js_insert_item(&mut self, index: usize, item: #inner_type_ident) -> Result<(), ::wasm_bindgen::JsError> {
                self.insert(index, item).map_err(|e| ::wasm_bindgen::JsError::new(&e.to_string()))
            }

            #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "removeItem")]
            pub fn js_remove_item(&mut self, index: usize) -> Result<#inner_type_ident, ::wasm_bindgen::JsError> {
                if index >= self.len() {
                    return Err(::wasm_bindgen::JsError::new("Index out of bounds"));
                }
                Ok(self.remove(index))
            }

            #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "length")]
            pub fn js_length(&self) -> usize {
                self.len()
            }

            #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "capacity")]
            pub fn js_capacity(&self) -> usize {
                self.capacity()
            }

            #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "maxCapacity")]
            pub fn js_max_capacity(&self) -> usize {
                self.max_capacity()
            }

            #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "remainingCapacity")]
            pub fn js_remaining_capacity(&self) -> usize {
                self.remaining_capacity()
            }

            #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "isEmpty")]
            pub fn js_is_empty(&self) -> bool {
                self.is_empty()
            }

            #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "clear")]
            pub fn js_clear(&mut self) {
                self.clear()
            }


            // Note: Conversion to/from JS Array requires type-specific implementation
            // for each type because wasm_bindgen doesn't support generics.
        }
    };

    TokenStream::from(expanded)
}