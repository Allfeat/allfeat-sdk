//! Type transformation utilities for converting between std and runtime types.
//!
//! This module handles the core logic of transforming Rust types from their std
//! form (`String`, `Vec<T>`) to their runtime form (`BoundedVec<u8, ConstU32<N>>`, etc.)
//! based on the compilation features.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericArgument, Path, PathArguments, Type, TypePath, TypeReference};

use crate::attribute::RuntimeBound;
use crate::error::{MacroError, MacroResult};

/// Handles type transformations between std and runtime modes
pub struct TypeTransformer;

impl TypeTransformer {
    /// Transforms a type for runtime mode if it's a supported transformable type
    ///
    /// # Supported Transformations
    /// - `String` → `BoundedVec<u8, ConstU32<N>>`
    /// - `Vec<T>` → `BoundedVec<T, ConstU32<N>>`
    /// - `Option<String>` → `Option<BoundedVec<u8, ConstU32<N>>>`
    /// - `Option<Vec<T>>` → `Option<BoundedVec<T, ConstU32<N>>>`
    /// - `&str` → `BoundedVec<u8, ConstU32<N>>`
    ///
    /// # Arguments
    /// - `ty`: The type to transform
    /// - `bound`: The runtime bound to apply
    /// - `as_runtime_type`: Optional AsRuntimeType annotation for inner type transformation
    ///
    /// # Returns
    /// A token stream representing the transformed type, or the original type if no transformation is needed
    pub fn transform_type_for_runtime(
        ty: &Type,
        bound: &RuntimeBound,
        as_runtime_type: &Option<crate::attribute::AsRuntimeType>,
    ) -> TokenStream {
        match ty {
            Type::Path(type_path) => Self::transform_path_type(type_path, bound, as_runtime_type),
            Type::Reference(type_ref) => Self::transform_reference_type(type_ref, bound),
            _ => {
                // No transformation needed for other types
                quote! { #ty }
            }
        }
    }

    /// Transforms a type to its runtime equivalent without bound (for fields that reference other MIDDS types)
    ///
    /// # Supported Transformations
    /// - `Iswc` → `RuntimeIswc` (with path context)
    /// - `MusicalWork` → `RuntimeMusicalWork`
    /// - `Track` → `RuntimeTrack`
    /// - `Release` → `RuntimeRelease`
    /// - `Option<Iswc>` → `Option<RuntimeIswc>`
    /// - etc.
    ///
    /// # Arguments
    /// - `ty`: The type to transform
    /// - `as_runtime_type`: Optional AsRuntimeType annotation with path info
    ///
    /// # Returns
    /// A token stream representing the runtime type, or the original type if no transformation is known
    pub fn transform_type_to_runtime_equivalent(
        ty: &Type,
        as_runtime_type: &Option<crate::attribute::AsRuntimeType>,
    ) -> TokenStream {
        match ty {
            Type::Path(type_path) => {
                Self::transform_path_to_runtime_equivalent(type_path, as_runtime_type)
            }
            _ => {
                // No transformation needed for other types
                quote! { #ty }
            }
        }
    }

    /// Transforms path types to their runtime equivalent (for MIDDS types)
    fn transform_path_to_runtime_equivalent(
        type_path: &TypePath,
        as_runtime_type: &Option<crate::attribute::AsRuntimeType>,
    ) -> TokenStream {
        let path = &type_path.path;

        // Handle Option<T> type with recursive transformation
        if let Some(inner_type) = Self::extract_option_inner_type(path) {
            let transformed_inner =
                Self::transform_type_to_runtime_equivalent(inner_type, as_runtime_type);
            return quote! { Option<#transformed_inner> };
        }

        // Check if this type should be transformed to Runtime variant
        if let Some(ident) = path.get_ident() {
            // Only transform specific MIDDS types that actually have runtime variants
            // Do NOT transform simple type aliases like MiddsId (which is just u64)
            match ident.to_string().as_str() {
                "Iswc"
                | "MusicalWork"
                | "MusicalWorkType"
                | "Ean"
                | "ClassicalInfo"
                | "ReleaseTitle"
                | "CoverContributorName"
                | "Isrc"
                | "Track"
                | "TrackTitle"
                | "TrackVersion" => {
                    let runtime_ident = syn::Ident::new(&format!("Runtime{}", ident), ident.span());

                    // Use path context from as_runtime_type annotation
                    if let Some(as_rt) = as_runtime_type {
                        if let Some(module_path) = &as_rt.path {
                            // #[as_runtime_type(path = "iswc")] → iswc::RuntimeIswc (types are now side by side)
                            let module_ident = syn::Ident::new(module_path, ident.span());
                            return quote! { #module_ident::#runtime_ident };
                        } else {
                            // #[as_runtime_type] → RuntimeIswc (same scope, side by side)
                            return quote! { #runtime_ident };
                        }
                    }

                    // Fallback to same scope
                    return quote! { #runtime_ident };
                }
                // Simple types like MiddsId (u64), Language, Key, etc. should not be transformed
                _ => {}
            }
        }

        // No transformation needed
        quote! { #type_path }
    }

    /// Transforms path types (String, Vec<T>, Option<T>)
    fn transform_path_type(
        type_path: &TypePath,
        bound: &RuntimeBound,
        as_runtime_type: &Option<crate::attribute::AsRuntimeType>,
    ) -> TokenStream {
        let path = &type_path.path;

        // Handle simple String type
        if Self::is_string_path(path) {
            return Self::generate_bounded_string_type(bound);
        }

        // Handle Vec<T> type
        if let Some(inner_type) = Self::extract_vec_inner_type(path) {
            return Self::generate_bounded_vec_type(inner_type, bound, as_runtime_type);
        }

        // Handle Option<T> type with recursive transformation
        if let Some(inner_type) = Self::extract_option_inner_type(path) {
            let transformed_inner =
                Self::transform_type_for_runtime(inner_type, bound, as_runtime_type);
            return quote! { Option<#transformed_inner> };
        }

        // No transformation needed
        quote! { #type_path }
    }

    /// Transforms reference types (&str)
    fn transform_reference_type(type_ref: &TypeReference, bound: &RuntimeBound) -> TokenStream {
        // Handle &str -> BoundedVec<u8, ConstU32<N>>
        if let Type::Path(type_path) = &*type_ref.elem
            && type_path.path.is_ident("str")
        {
            return Self::generate_bounded_string_type(bound);
        }

        // No transformation needed
        quote! { #type_ref }
    }

    /// Checks if a path represents the String type
    fn is_string_path(path: &Path) -> bool {
        path.is_ident("String")
    }

    /// Extracts the inner type from Vec<T> if the path represents a Vec
    fn extract_vec_inner_type(path: &Path) -> Option<&Type> {
        let segment = path.segments.last()?;

        if segment.ident != "Vec" {
            return None;
        }

        match &segment.arguments {
            PathArguments::AngleBracketed(args) => {
                if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                    Some(inner_type)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Extracts the inner type from Option<T> if the path represents an Option
    fn extract_option_inner_type(path: &Path) -> Option<&Type> {
        let segment = path.segments.last()?;

        if segment.ident != "Option" {
            return None;
        }

        match &segment.arguments {
            PathArguments::AngleBracketed(args) => {
                if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                    Some(inner_type)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Generates a BoundedVec<u8, ConstU32<N>> type for String transformations
    fn generate_bounded_string_type(bound: &RuntimeBound) -> TokenStream {
        let bound_literal = bound.literal();
        quote! {
            frame_support::BoundedVec<u8, frame_support::traits::ConstU32<#bound_literal>>
        }
    }

    /// Generates a BoundedVec<T, ConstU32<N>> type for Vec<T> transformations
    fn generate_bounded_vec_type(
        inner_type: &Type,
        bound: &RuntimeBound,
        as_runtime_type: &Option<crate::attribute::AsRuntimeType>,
    ) -> TokenStream {
        let bound_literal = bound.literal();

        // Transform the inner type to its runtime equivalent
        let transformed_inner_type =
            Self::transform_type_to_runtime_equivalent(inner_type, as_runtime_type);

        quote! {
            frame_support::BoundedVec<#transformed_inner_type, frame_support::traits::ConstU32<#bound_literal>>
        }
    }

    /// Checks if a type requires a runtime bound when used in runtime mode
    pub fn requires_runtime_bound(ty: &Type) -> bool {
        match ty {
            Type::Path(type_path) => {
                let path = &type_path.path;

                // String requires a bound
                if Self::is_string_path(path) {
                    return true;
                }

                // Vec<T> requires a bound
                if Self::extract_vec_inner_type(path).is_some() {
                    return true;
                }

                // Option<T> requires a bound if T requires one
                if let Some(inner_type) = Self::extract_option_inner_type(path) {
                    return Self::requires_runtime_bound(inner_type);
                }

                // &str requires a bound
                false // This is handled in Type::Reference
            }
            Type::Reference(type_ref) => {
                // &str requires a bound
                if let Type::Path(type_path) = &*type_ref.elem
                    && type_path.path.is_ident("str")
                {
                    return true;
                }

                false
            }
            _ => false,
        }
    }

    /// Gets a human-readable string representation of a type for error messages
    pub fn type_to_string(ty: &Type) -> String {
        match ty {
            Type::Path(type_path) => {
                if type_path.path.segments.len() == 1 {
                    type_path.path.segments[0].ident.to_string()
                } else {
                    // For complex paths, use the full path
                    quote! { #type_path }.to_string()
                }
            }
            Type::Reference(type_ref) => {
                format!("&{}", Self::type_to_string(&type_ref.elem))
            }
            _ => quote! { #ty }.to_string(),
        }
    }
}

/// Validates that the bound is appropriate for the given type
pub fn validate_bound_for_type(ty: &Type, _bound: &RuntimeBound) -> MacroResult<()> {
    // For now, we allow any positive bound for supported types
    // Future enhancements could add type-specific validation
    // (e.g., String bounds should be reasonable for UTF-8 text)

    if !TypeTransformer::requires_runtime_bound(ty) {
        return Err(MacroError::unsupported_bound_type(
            ty,
            &TypeTransformer::type_to_string(ty),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_quote, LitInt};

    fn create_test_bound(value: u32) -> RuntimeBound {
        let lit: LitInt = syn::parse_str(&value.to_string()).unwrap();
        RuntimeBound::new(lit).unwrap()
    }

    #[test]
    fn test_transform_string_type() {
        let ty: Type = parse_quote! { String };
        let bound = create_test_bound(256);

        let result = TypeTransformer::transform_type_for_runtime(&ty, &bound, &None);
        let result_str = result.to_string();

        assert!(result_str.contains("BoundedVec"));
        assert!(result_str.contains("u8"));
        assert!(result_str.contains("256"));
    }

    #[test]
    fn test_transform_vec_type() {
        let ty: Type = parse_quote! { Vec<u32> };
        let bound = create_test_bound(128);

        let result = TypeTransformer::transform_type_for_runtime(&ty, &bound, &None);
        let result_str = result.to_string();

        assert!(result_str.contains("BoundedVec"));
        assert!(result_str.contains("u32"));
        assert!(result_str.contains("128"));
    }

    #[test]
    fn test_transform_option_string_type() {
        let ty: Type = parse_quote! { Option<String> };
        let bound = create_test_bound(64);

        let result = TypeTransformer::transform_type_for_runtime(&ty, &bound, &None);
        let result_str = result.to_string();

        assert!(result_str.contains("Option"));
        assert!(result_str.contains("BoundedVec"));
        assert!(result_str.contains("u8"));
        assert!(result_str.contains("64"));
    }

    #[test]
    fn test_transform_str_reference() {
        let ty: Type = parse_quote! { &str };
        let bound = create_test_bound(32);

        let result = TypeTransformer::transform_type_for_runtime(&ty, &bound, &None);
        let result_str = result.to_string();

        assert!(result_str.contains("BoundedVec"));
        assert!(result_str.contains("u8"));
        assert!(result_str.contains("32"));
    }

    #[test]
    fn test_no_transform_for_primitive() {
        let ty: Type = parse_quote! { u32 };
        let bound = create_test_bound(100);

        let result = TypeTransformer::transform_type_for_runtime(&ty, &bound, &None);
        let result_str = result.to_string();

        assert_eq!(result_str, "u32");
    }

    #[test]
    fn test_requires_runtime_bound() {
        let string_ty: Type = parse_quote! { String };
        assert!(TypeTransformer::requires_runtime_bound(&string_ty));

        let vec_ty: Type = parse_quote! { Vec<u32> };
        assert!(TypeTransformer::requires_runtime_bound(&vec_ty));

        let primitive_ty: Type = parse_quote! { u32 };
        assert!(!TypeTransformer::requires_runtime_bound(&primitive_ty));
    }

    #[test]
    fn test_type_to_string() {
        let string_ty: Type = parse_quote! { String };
        assert_eq!(TypeTransformer::type_to_string(&string_ty), "String");

        let vec_ty: Type = parse_quote! { Vec<u32> };
        let vec_str = TypeTransformer::type_to_string(&vec_ty);
        assert!(vec_str.contains("Vec"));

        let ref_ty: Type = parse_quote! { &str };
        assert_eq!(TypeTransformer::type_to_string(&ref_ty), "&str");
    }
}
