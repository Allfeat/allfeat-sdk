//! Struct handling logic for the MIDDS V2 procedural macro.
//!
//! This module provides specialized handling for different types of structs:
//! unit structs, tuple structs, and named field structs.

use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::{DataStruct, Fields};

use crate::attribute::AttributeParser;
use crate::error::{MacroError, MacroResult};
use crate::generate::{field_gen, struct_gen, GenerationConfig};
use crate::transform::{validate_bound_for_type, TypeTransformer};

/// Handles processing of struct definitions
pub struct StructHandler;

impl StructHandler {
    /// Processes a struct and generates the appropriate dual-mode code
    pub fn process_struct(
        config: &GenerationConfig,
        data_struct: &DataStruct,
    ) -> MacroResult<TokenStream> {
        match &data_struct.fields {
            Fields::Named(fields_named) => {
                Self::process_named_fields(config, &fields_named.named.iter().collect::<Vec<_>>())
            }
            Fields::Unnamed(fields_unnamed) => Self::process_unnamed_fields(
                config,
                &fields_unnamed.unnamed.iter().collect::<Vec<_>>(),
            ),
            Fields::Unit => {
                // Unit structs don't have fields to process
                Ok(struct_gen::generate_struct(config, &[], &[], true))
            }
        }
    }

    /// Processes named fields (struct with `{ field: type }` syntax)
    fn process_named_fields(
        config: &GenerationConfig,
        fields: &[&syn::Field],
    ) -> MacroResult<TokenStream> {
        let mut runtime_fields = Vec::new();
        let mut native_fields = Vec::new();
        let mut errors = Vec::new();

        for field in fields {
            match Self::process_named_field(field) {
                Ok((runtime_field, native_field)) => {
                    runtime_fields.push(runtime_field);
                    native_fields.push(native_field);
                }
                Err(error) => errors.push(error),
            }
        }

        if !errors.is_empty() {
            // Return the first error instead of a generic error
            return Err(errors.into_iter().next().unwrap());
        }

        Ok(struct_gen::generate_struct(
            config,
            &runtime_fields,
            &native_fields,
            false,
        ))
    }

    /// Processes unnamed fields (tuple struct with `(type, type)` syntax)
    fn process_unnamed_fields(
        config: &GenerationConfig,
        fields: &[&syn::Field],
    ) -> MacroResult<TokenStream> {
        let mut runtime_fields = Vec::new();
        let mut native_fields = Vec::new();
        let mut errors = Vec::new();

        for field in fields {
            match Self::process_unnamed_field(field) {
                Ok((runtime_field, native_field)) => {
                    runtime_fields.push(runtime_field);
                    native_fields.push(native_field);
                }
                Err(error) => errors.push(error),
            }
        }

        if !errors.is_empty() {
            // Return the first error instead of a generic error
            return Err(errors.into_iter().next().unwrap());
        }

        Ok(struct_gen::generate_tuple_struct(
            config,
            &runtime_fields,
            &native_fields,
        ))
    }

    /// Processes a single named field
    fn process_named_field(field: &syn::Field) -> MacroResult<(TokenStream, TokenStream)> {
        let field_name =
            field
                .ident
                .as_ref()
                .ok_or_else(|| MacroError::UnsupportedDataStructure {
                    span: field.span(),
                    attempted: "field without name in named struct".to_string(),
                })?;

        let field_type = &field.ty;
        let field_vis = &field.vis;
        let field_attrs = &field.attrs;

        // Validate attributes
        AttributeParser::validate_attributes(field_attrs)?;

        // Extract runtime bound if present
        let maybe_bound = AttributeParser::extract_runtime_bound(field_attrs);

        // Filter out runtime_bound attributes for final output
        let filtered_attrs = AttributeParser::filter_runtime_bound_attrs(field_attrs);

        let (runtime_type, native_type) = if let Some(bound_result) = maybe_bound {
            let bound = bound_result?;

            // Validate that this type supports runtime bounds
            validate_bound_for_type(field_type, &bound)?;

            // Transform the type for runtime mode
            let runtime_type_tokens =
                TypeTransformer::transform_type_for_runtime(field_type, &bound);

            (runtime_type_tokens, quote::quote! { #field_type })
        } else {
            // Check if this type should have a bound but doesn't
            if TypeTransformer::requires_runtime_bound(field_type) {
                return Err(MacroError::missing_runtime_bound(
                    field,
                    &TypeTransformer::type_to_string(field_type),
                ));
            }

            // No transformation needed
            (quote::quote! { #field_type }, quote::quote! { #field_type })
        };

        let runtime_field =
            field_gen::generate_named_field(field_name, &runtime_type, field_vis, &filtered_attrs);

        let native_field =
            field_gen::generate_named_field(field_name, &native_type, field_vis, &filtered_attrs);

        Ok((runtime_field, native_field))
    }

    /// Processes a single unnamed field
    fn process_unnamed_field(field: &syn::Field) -> MacroResult<(TokenStream, TokenStream)> {
        let field_type = &field.ty;
        let field_attrs = &field.attrs;

        // Validate attributes
        AttributeParser::validate_attributes(field_attrs)?;

        // Extract runtime bound if present
        let maybe_bound = AttributeParser::extract_runtime_bound(field_attrs);

        // Filter out runtime_bound attributes for final output
        let filtered_attrs = AttributeParser::filter_runtime_bound_attrs(field_attrs);

        let (runtime_type, native_type) = if let Some(bound_result) = maybe_bound {
            let bound = bound_result?;

            // Validate that this type supports runtime bounds
            validate_bound_for_type(field_type, &bound)?;

            // Transform the type for runtime mode
            let runtime_type_tokens =
                TypeTransformer::transform_type_for_runtime(field_type, &bound);

            (runtime_type_tokens, quote::quote! { #field_type })
        } else {
            // Check if this type should have a bound but doesn't
            if TypeTransformer::requires_runtime_bound(field_type) {
                return Err(MacroError::missing_runtime_bound(
                    field,
                    &TypeTransformer::type_to_string(field_type),
                ));
            }

            // No transformation needed
            (quote::quote! { #field_type }, quote::quote! { #field_type })
        };

        let runtime_field = field_gen::generate_unnamed_field(&runtime_type, &filtered_attrs);
        let native_field = field_gen::generate_unnamed_field(&native_type, &filtered_attrs);

        Ok((runtime_field, native_field))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Span;
    use syn::{parse_quote, Ident};

    fn create_test_config() -> GenerationConfig {
        GenerationConfig::new(
            Ident::new("TestStruct", Span::call_site()),
            parse_quote! { pub },
            syn::Generics::default(),
            vec![],
        )
    }

    #[test]
    fn test_process_unit_struct() {
        let config = create_test_config();
        let data_struct = syn::DataStruct {
            struct_token: syn::token::Struct::default(),
            fields: syn::Fields::Unit,
            semi_token: Some(syn::token::Semi::default()),
        };

        let result = StructHandler::process_struct(&config, &data_struct);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("struct TestStruct"));
    }

    #[test]
    fn test_process_named_field_with_bound() {
        let field: syn::Field = parse_quote! {
            #[runtime_bound(256)]
            pub title: String
        };

        let result = StructHandler::process_named_field(&field);
        assert!(result.is_ok());

        let (runtime_field, native_field) = result.unwrap();
        let runtime_str = runtime_field.to_string();
        let native_str = native_field.to_string();

        assert!(runtime_str.contains("BoundedVec"));
        assert!(native_str.contains("String"));
    }

    #[test]
    fn test_process_named_field_missing_bound() {
        let field: syn::Field = parse_quote! {
            pub title: String
        };

        let result = StructHandler::process_named_field(&field);
        assert!(result.is_err());
    }

    #[test]
    fn test_process_unnamed_field_with_bound() {
        let field: syn::Field = parse_quote! {
            #[runtime_bound(128)]
            String
        };

        let result = StructHandler::process_unnamed_field(&field);
        assert!(result.is_ok());

        let (runtime_field, native_field) = result.unwrap();
        let runtime_str = runtime_field.to_string();
        let native_str = native_field.to_string();

        assert!(runtime_str.contains("BoundedVec"));
        assert!(native_str.contains("String"));
    }

    #[test]
    fn test_process_field_no_transform_needed() {
        let field: syn::Field = parse_quote! {
            pub id: u64
        };

        let result = StructHandler::process_named_field(&field);
        assert!(result.is_ok());

        let (runtime_field, native_field) = result.unwrap();
        let runtime_str = runtime_field.to_string();
        let native_str = native_field.to_string();

        // Both should be the same since u64 doesn't need transformation
        assert!(runtime_str.contains("u64"));
        assert!(native_str.contains("u64"));
        assert!(!runtime_str.contains("BoundedVec"));
    }
}

