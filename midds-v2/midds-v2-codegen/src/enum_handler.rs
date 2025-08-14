//! Enum handling logic for the MIDDS V2 procedural macro.
//!
//! This module provides specialized handling for different types of enum variants:
//! unit variants, tuple variants, and struct variants.

use proc_macro2::TokenStream;
use syn::{DataEnum, Fields, Variant};

use crate::attribute::AttributeParser;
use crate::error::{MacroError, MacroResult};
use crate::generate::{enum_gen, variant_gen, GenerationConfig};
use crate::transform::{validate_bound_for_type, TypeTransformer};

/// Handles processing of enum definitions
pub struct EnumHandler;

impl EnumHandler {
    /// Processes an enum and generates the appropriate dual-mode code
    pub fn process_enum(
        config: &GenerationConfig,
        data_enum: &DataEnum,
    ) -> MacroResult<TokenStream> {
        let mut runtime_variants = Vec::new();
        let mut std_variants = Vec::new();
        let mut errors = Vec::new();

        // Simple detection: enum has transformations if any variant has #[runtime_bound]
        let has_transformations = data_enum
            .variants
            .iter()
            .any(|variant| AttributeParser::extract_runtime_bound(&variant.attrs).is_some());

        for variant in &data_enum.variants {
            match Self::process_variant(variant) {
                Ok((runtime_variant, std_variant)) => {
                    runtime_variants.push(runtime_variant);
                    std_variants.push(std_variant);
                }
                Err(error) => errors.push(error),
            }
        }

        if !errors.is_empty() {
            // Return the first error instead of a generic error
            return Err(errors.into_iter().next().unwrap());
        }

        Ok(enum_gen::generate_enum(
            config,
            &runtime_variants,
            &std_variants,
            has_transformations,
        ))
    }

    /// Processes a single enum variant
    fn process_variant(variant: &Variant) -> MacroResult<(TokenStream, TokenStream)> {
        let variant_name = &variant.ident;
        let variant_attrs = &variant.attrs;

        // Validate attributes
        AttributeParser::validate_attributes(variant_attrs)?;

        // Extract runtime bound if present (applies to all fields in the variant)
        let maybe_bound = AttributeParser::extract_runtime_bound(variant_attrs);

        // Filter out runtime_bound attributes for final output
        let filtered_attrs = AttributeParser::filter_runtime_bound_attrs(variant_attrs);

        match &variant.fields {
            Fields::Unit => {
                // Unit variant - no fields to process, no transformations
                let runtime_variant =
                    variant_gen::generate_unit_variant(variant_name, &filtered_attrs);
                let std_variant = variant_gen::generate_unit_variant(variant_name, &filtered_attrs);

                Ok((runtime_variant, std_variant))
            }
            Fields::Unnamed(fields) => {
                Self::process_tuple_variant(variant_name, fields, &filtered_attrs, maybe_bound)
            }
            Fields::Named(fields) => {
                // For now, we don't transform named fields in enum variants
                // This could be extended in the future if needed
                let fields_tokens = &fields.named;
                let runtime_variant = variant_gen::generate_struct_variant(
                    variant_name,
                    &quote::quote! { #fields_tokens },
                    &filtered_attrs,
                );
                let std_variant = variant_gen::generate_struct_variant(
                    variant_name,
                    &quote::quote! { #fields_tokens },
                    &filtered_attrs,
                );

                Ok((runtime_variant, std_variant))
            }
        }
    }

    /// Processes a tuple variant (variant with unnamed fields)
    fn process_tuple_variant(
        variant_name: &syn::Ident,
        fields: &syn::FieldsUnnamed,
        filtered_attrs: &[&syn::Attribute],
        maybe_bound: Option<MacroResult<crate::attribute::RuntimeBound>>,
    ) -> MacroResult<(TokenStream, TokenStream)> {
        if let Some(bound_result) = maybe_bound {
            let bound = bound_result?;

            // Transform all fields in the variant using the same bound
            let mut runtime_field_types = Vec::new();
            let mut std_field_types = Vec::new();

            for field in &fields.unnamed {
                let field_type = &field.ty;

                // Only transform types that require bounds
                if TypeTransformer::requires_runtime_bound(field_type) {
                    // Validate that this type supports runtime bounds
                    validate_bound_for_type(field_type, &bound)?;

                    // Transform the type for runtime mode
                    let runtime_type =
                        TypeTransformer::transform_type_for_runtime(field_type, &bound, &None);
                    runtime_field_types.push(runtime_type);
                } else {
                    // Keep the original type
                    runtime_field_types.push(quote::quote! { #field_type });
                }

                // Std version always uses original type
                std_field_types.push(quote::quote! { #field_type });
            }

            let runtime_variant = variant_gen::generate_tuple_variant(
                variant_name,
                &runtime_field_types,
                filtered_attrs,
            );

            let std_variant =
                variant_gen::generate_tuple_variant(variant_name, &std_field_types, filtered_attrs);

            Ok((runtime_variant, std_variant))
        } else {
            // No bound specified - check if any field requires one
            for field in &fields.unnamed {
                if TypeTransformer::requires_runtime_bound(&field.ty) {
                    return Err(MacroError::missing_runtime_bound(
                        field,
                        &TypeTransformer::type_to_string(&field.ty),
                    ));
                }
            }

            // No transformation needed - keep original fields
            let field_types: Vec<_> = fields
                .unnamed
                .iter()
                .map(|field| {
                    let field_type = &field.ty;
                    quote::quote! { #field_type }
                })
                .collect();

            let runtime_variant =
                variant_gen::generate_tuple_variant(variant_name, &field_types, filtered_attrs);
            let std_variant =
                variant_gen::generate_tuple_variant(variant_name, &field_types, filtered_attrs);

            Ok((runtime_variant, std_variant))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Span;
    use syn::{parse_quote, Ident};

    fn create_test_config() -> GenerationConfig {
        GenerationConfig::new(
            Ident::new("TestEnum", Span::call_site()),
            parse_quote! { pub },
            syn::Generics::default(),
            vec![],
        )
    }

    #[test]
    fn test_process_unit_variant() {
        let variant: Variant = parse_quote! { Original };

        let result = EnumHandler::process_variant(&variant);
        assert!(result.is_ok());

        let (runtime_variant, std_variant) = result.unwrap();
        let runtime_str = runtime_variant.to_string();
        let std_str = std_variant.to_string();

        assert!(runtime_str.contains("Original"));
        assert!(std_str.contains("Original"));
        assert!(!runtime_str.contains("BoundedVec"));
    }

    #[test]
    fn test_process_tuple_variant_with_bound() {
        let variant: Variant = parse_quote! {
            #[runtime_bound(256)]
            WithData(String, u32)
        };

        let result = EnumHandler::process_variant(&variant);
        assert!(result.is_ok());

        let (runtime_variant, std_variant) = result.unwrap();
        let runtime_str = runtime_variant.to_string();
        let std_str = std_variant.to_string();

        assert!(runtime_str.contains("WithData"));
        assert!(runtime_str.contains("BoundedVec")); // String should be transformed
        assert!(runtime_str.contains("u32")); // u32 should remain unchanged
        assert!(std_str.contains("String")); // Std should keep String
        assert!(std_str.contains("u32"));
    }

    #[test]
    fn test_process_tuple_variant_missing_bound() {
        let variant: Variant = parse_quote! {
            WithData(String, u32)
        };

        let result = EnumHandler::process_variant(&variant);
        assert!(result.is_err()); // Should error because String needs a bound
    }

    #[test]
    fn test_process_tuple_variant_no_bound_needed() {
        let variant: Variant = parse_quote! {
            WithData(u32, bool)
        };

        let result = EnumHandler::process_variant(&variant);
        assert!(result.is_ok()); // Should work because primitive types don't need bounds

        let (runtime_variant, std_variant) = result.unwrap();
        let runtime_str = runtime_variant.to_string();
        let std_str = std_variant.to_string();

        assert!(runtime_str.contains("u32"));
        assert!(runtime_str.contains("bool"));
        assert!(!runtime_str.contains("BoundedVec"));
        assert_eq!(runtime_str, std_str); // Should be identical
    }

    #[test]
    fn test_process_struct_variant() {
        let variant: Variant = parse_quote! {
            Named { id: u32, active: bool }
        };

        let result = EnumHandler::process_variant(&variant);
        assert!(result.is_ok());

        let (runtime_variant, std_variant) = result.unwrap();
        let runtime_str = runtime_variant.to_string();
        let std_str = std_variant.to_string();

        assert!(runtime_str.contains("Named"));
        assert!(runtime_str.contains("id"));
        assert!(runtime_str.contains("u32"));
        assert!(runtime_str.contains("active"));
        assert!(runtime_str.contains("bool"));
        assert_eq!(runtime_str, std_str); // Should be identical for struct variants
    }

    #[test]
    fn test_process_enum() {
        let config = create_test_config();
        // Create a simple test DataEnum manually
        use syn::punctuated::Punctuated;
        let mut variants = Punctuated::new();

        // Add Original variant
        variants.push(syn::Variant {
            attrs: vec![],
            ident: syn::Ident::new("Original", proc_macro2::Span::call_site()),
            fields: syn::Fields::Unit,
            discriminant: None,
        });

        // Add WithData variant with bound
        let bound_attr: syn::Attribute = parse_quote! { #[runtime_bound(128)] };
        variants.push(syn::Variant {
            attrs: vec![bound_attr],
            ident: syn::Ident::new("WithData", proc_macro2::Span::call_site()),
            fields: syn::Fields::Unnamed(syn::FieldsUnnamed {
                paren_token: syn::token::Paren::default(),
                unnamed: {
                    let mut fields = Punctuated::new();
                    fields.push(syn::Field {
                        attrs: vec![],
                        vis: syn::Visibility::Inherited,
                        mutability: syn::FieldMutability::None,
                        ident: None,
                        colon_token: None,
                        ty: parse_quote! { String },
                    });
                    fields.push(syn::Field {
                        attrs: vec![],
                        vis: syn::Visibility::Inherited,
                        mutability: syn::FieldMutability::None,
                        ident: None,
                        colon_token: None,
                        ty: parse_quote! { u32 },
                    });
                    fields
                },
            }),
            discriminant: None,
        });

        // Add Other variant
        variants.push(syn::Variant {
            attrs: vec![],
            ident: syn::Ident::new("Other", proc_macro2::Span::call_site()),
            fields: syn::Fields::Unnamed(syn::FieldsUnnamed {
                paren_token: syn::token::Paren::default(),
                unnamed: {
                    let mut fields = Punctuated::new();
                    fields.push(syn::Field {
                        attrs: vec![],
                        vis: syn::Visibility::Inherited,
                        mutability: syn::FieldMutability::None,
                        ident: None,
                        colon_token: None,
                        ty: parse_quote! { bool },
                    });
                    fields
                },
            }),
            discriminant: None,
        });

        let data_enum = syn::DataEnum {
            enum_token: syn::token::Enum::default(),
            brace_token: syn::token::Brace::default(),
            variants,
        };

        let result = EnumHandler::process_enum(&config, &data_enum);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();

        assert!(tokens_str.contains("enum TestEnum"));
        assert!(tokens_str.contains("Original"));
        assert!(tokens_str.contains("WithData"));
        assert!(tokens_str.contains("Other"));
        assert!(tokens_str.contains("BoundedVec")); // Should have transformation
    }
}

