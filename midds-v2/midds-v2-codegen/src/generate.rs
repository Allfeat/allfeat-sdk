//! Code generation utilities for the MIDDS V2 procedural macro.
//!
//! This module handles generating the final Rust code for both native and runtime modes,
//! including derive attributes and proper feature gating.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Generics, Ident, Visibility, WhereClause};

/// Generates derive attributes for runtime mode
pub fn generate_runtime_derives() -> TokenStream {
    quote! {
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
    }
}

/// Generates derive attributes for native mode
pub fn generate_native_derives() -> TokenStream {
    quote! {
        #[derive(Debug, Clone, PartialEq, Eq)]
    }
}

/// Configuration for code generation
#[derive(Debug, Clone)]
pub struct GenerationConfig {
    /// The name of the type being generated
    pub type_name: Ident,
    /// Visibility of the type
    pub visibility: Visibility,
    /// Generic parameters
    pub generics: Generics,
    /// Where clause for generics
    pub where_clause: Option<WhereClause>,
    /// Original attributes (excluding runtime_bound)
    pub attributes: Vec<Attribute>,
}

impl GenerationConfig {
    /// Creates a new generation config from parsed syntax elements
    pub fn new(
        type_name: Ident,
        visibility: Visibility,
        generics: Generics,
        attributes: Vec<Attribute>,
    ) -> Self {
        let where_clause = generics.where_clause.clone();
        
        Self {
            type_name,
            visibility,
            generics,
            where_clause,
            attributes,
        }
    }

    /// Gets the impl generics for the type
    pub fn impl_generics(&self) -> syn::ImplGenerics<'_> {
        self.generics.split_for_impl().0
    }

    /// Gets the type generics for the type
    pub fn type_generics(&self) -> syn::TypeGenerics<'_> {
        self.generics.split_for_impl().1
    }
}

/// Generates a complete type definition with both runtime and native variants
pub fn generate_dual_mode_type(
    config: &GenerationConfig,
    runtime_body: TokenStream,
    native_body: TokenStream,
) -> TokenStream {
    let GenerationConfig {
        visibility,
        attributes,
        ..
    } = config;

    let _impl_generics = config.impl_generics();
    let _type_generics = config.type_generics();
    let _where_clause = &config.where_clause;

    let runtime_derives = generate_runtime_derives();
    let native_derives = generate_native_derives();

    quote! {
        #[cfg(feature = "runtime")]
        #(#attributes)*
        #runtime_derives
        #visibility #runtime_body

        #[cfg(not(feature = "runtime"))]
        #(#attributes)*
        #native_derives
        #visibility #native_body
    }
}

/// Struct-specific code generation utilities
pub mod struct_gen {
    use super::*;
    use proc_macro2::TokenStream;
    use quote::quote;

    /// Generates a struct definition
    pub fn generate_struct(
        config: &GenerationConfig,
        runtime_fields: &[TokenStream],
        native_fields: &[TokenStream],
        is_unit: bool,
    ) -> TokenStream {
        let type_name = &config.type_name;

        let impl_generics = config.impl_generics();
        let _type_generics = config.type_generics();
        let where_clause = &config.where_clause;

        if is_unit {
            let runtime_body = quote! { struct #type_name #impl_generics #where_clause; };
            let native_body = quote! { struct #type_name #impl_generics #where_clause; };
            return generate_dual_mode_type(config, runtime_body, native_body);
        }

        let runtime_body = quote! {
            struct #type_name #impl_generics #where_clause {
                #(#runtime_fields,)*
            }
        };

        let native_body = quote! {
            struct #type_name #impl_generics #where_clause {
                #(#native_fields,)*
            }
        };

        generate_dual_mode_type(config, runtime_body, native_body)
    }

    /// Generates a tuple struct definition
    pub fn generate_tuple_struct(
        config: &GenerationConfig,
        runtime_fields: &[TokenStream],
        native_fields: &[TokenStream],
    ) -> TokenStream {
        let type_name = &config.type_name;

        let impl_generics = config.impl_generics();
        let _type_generics = config.type_generics();
        let where_clause = &config.where_clause;

        let runtime_body = quote! {
            struct #type_name #impl_generics (#(#runtime_fields),*) #where_clause;
        };

        let native_body = quote! {
            struct #type_name #impl_generics (#(#native_fields),*) #where_clause;
        };

        generate_dual_mode_type(config, runtime_body, native_body)
    }
}

/// Enum-specific code generation utilities
pub mod enum_gen {
    use super::*;
    use proc_macro2::TokenStream;
    use quote::quote;

    /// Generates an enum definition
    pub fn generate_enum(
        config: &GenerationConfig,
        runtime_variants: &[TokenStream],
        native_variants: &[TokenStream],
    ) -> TokenStream {
        let type_name = &config.type_name;

        let impl_generics = config.impl_generics();
        let _type_generics = config.type_generics();
        let where_clause = &config.where_clause;

        let runtime_body = quote! {
            enum #type_name #impl_generics #where_clause {
                #(#runtime_variants),*
            }
        };

        let native_body = quote! {
            enum #type_name #impl_generics #where_clause {
                #(#native_variants),*
            }
        };

        generate_dual_mode_type(config, runtime_body, native_body)
    }
}

/// Field generation utilities
pub mod field_gen {
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{Attribute, Ident, Visibility};

    /// Generates a named field
    pub fn generate_named_field(
        field_name: &Ident,
        field_type: &TokenStream,
        visibility: &Visibility,
        attributes: &[&Attribute],
    ) -> TokenStream {
        quote! {
            #(#attributes)*
            #visibility #field_name: #field_type
        }
    }

    /// Generates an unnamed field (for tuple structs)
    pub fn generate_unnamed_field(
        field_type: &TokenStream,
        attributes: &[&Attribute],
    ) -> TokenStream {
        quote! {
            #(#attributes)*
            #field_type
        }
    }
}

/// Variant generation utilities for enums
pub mod variant_gen {
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{Attribute, Ident};

    /// Generates a unit variant
    pub fn generate_unit_variant(
        variant_name: &Ident,
        attributes: &[&Attribute],
    ) -> TokenStream {
        quote! {
            #(#attributes)*
            #variant_name
        }
    }

    /// Generates a tuple variant
    pub fn generate_tuple_variant(
        variant_name: &Ident,
        fields: &[TokenStream],
        attributes: &[&Attribute],
    ) -> TokenStream {
        quote! {
            #(#attributes)*
            #variant_name(#(#fields),*)
        }
    }

    /// Generates a struct variant
    pub fn generate_struct_variant(
        variant_name: &Ident,
        fields: &TokenStream,
        attributes: &[&Attribute],
    ) -> TokenStream {
        quote! {
            #(#attributes)*
            #variant_name { #fields }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_quote, Ident};
    use proc_macro2::Span;

    fn create_test_config() -> GenerationConfig {
        GenerationConfig::new(
            Ident::new("TestStruct", Span::call_site()),
            parse_quote! { pub },
            syn::Generics::default(),
            vec![],
        )
    }

    #[test]
    fn test_generate_runtime_derives() {
        let derives = generate_runtime_derives();
        let derives_str = derives.to_string();
        
        assert!(derives_str.contains("Encode"));
        assert!(derives_str.contains("Decode"));
        assert!(derives_str.contains("TypeInfo"));
        assert!(derives_str.contains("MaxEncodedLen"));
        assert!(derives_str.contains("Debug"));
    }

    #[test]
    fn test_generate_native_derives() {
        let derives = generate_native_derives();
        let derives_str = derives.to_string();
        
        assert!(derives_str.contains("Debug"));
        assert!(derives_str.contains("Clone"));
        assert!(derives_str.contains("PartialEq"));
        assert!(derives_str.contains("Eq"));
    }

    #[test]
    fn test_generate_dual_mode_type() {
        let config = create_test_config();
        let runtime_body = quote! { struct TestStruct { x: u32 } };
        let native_body = quote! { struct TestStruct { x: u32 } };
        
        let result = generate_dual_mode_type(&config, runtime_body, native_body);
        let result_str = result.to_string();
        
        assert!(result_str.contains("# [cfg (feature = \"runtime\")]"));
        assert!(result_str.contains("# [cfg (not (feature = \"runtime\"))]"));
        assert!(result_str.contains("Encode")); // Runtime derive
    }

    #[test]
    fn test_struct_generation() {
        let config = create_test_config();
        let runtime_fields = vec![quote! { pub x: u32 }];
        let native_fields = vec![quote! { pub x: u32 }];
        
        let result = struct_gen::generate_struct(&config, &runtime_fields, &native_fields, false);
        let result_str = result.to_string();
        
        assert!(result_str.contains("struct TestStruct"));
        assert!(result_str.contains("pub x : u32"));
    }

    #[test]
    fn test_tuple_struct_generation() {
        let config = create_test_config();
        let runtime_fields = vec![quote! { u32 }];
        let native_fields = vec![quote! { u32 }];
        
        let result = struct_gen::generate_tuple_struct(&config, &runtime_fields, &native_fields);
        let result_str = result.to_string();
        
        assert!(result_str.contains("struct TestStruct ("));
        assert!(result_str.contains("u32"));
    }
}