//! Code generation utilities for the MIDDS V2 procedural macro.
//!
//! This module handles generating the final Rust code for both std and runtime modes,
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

/// Generates derive attributes for std mode
pub fn generate_std_derives() -> TokenStream {
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

/// Generates separate types: original (std/web) and RuntimeXXX side by side
fn generate_separate_types(
    config: &GenerationConfig,
    runtime_body: TokenStream,
    std_body: TokenStream,
) -> TokenStream {
    use proc_macro2::Span;
    use syn::Ident;

    let GenerationConfig {
        visibility,
        attributes,
        type_name,
        ..
    } = config;

    let _impl_generics = config.impl_generics();
    let _type_generics = config.type_generics();
    let _where_clause = &config.where_clause;

    // Create Runtime prefixed type name (unused but kept for potential future use)
    let _runtime_type_name = Ident::new(&format!("Runtime{}", type_name), Span::call_site());

    let runtime_derives = generate_runtime_derives();
    let std_derives = generate_std_derives();

    quote! {
        // Original type for std and web features (requires std for String/Vec)
        // Always available when std is present, can coexist with runtime types
        #[cfg(feature = "std")]
        #(#attributes)*
        #std_derives
        #visibility #std_body

        // Runtime type generated side by side with the original type
        #[cfg(feature = "runtime")]
        #(#attributes)*
        #runtime_derives
        #visibility #runtime_body
    }
}

/// Generates the same type with different derives for std vs runtime modes
/// Used for enums and simple structs that don't need type transformation
pub fn generate_same_type_different_derives(
    config: &GenerationConfig,
    type_body: TokenStream,
) -> TokenStream {
    let GenerationConfig {
        visibility,
        attributes,
        ..
    } = config;

    quote! {
        // Same type for both modes with conditional derives
        #[cfg_attr(
            not(feature = "runtime"),
            derive(Debug, Clone, PartialEq, Eq, Copy)
        )]
        #[cfg_attr(
            feature = "runtime",
            derive(
                parity_scale_codec::Encode,
                parity_scale_codec::Decode,
                parity_scale_codec::DecodeWithMemTracking,
                scale_info::TypeInfo,
                parity_scale_codec::MaxEncodedLen,
                Debug,
                Clone,
                PartialEq,
                Eq,
                Copy
            )
        )]
        #(#attributes)*
        #visibility #type_body
    }
}

/// Struct-specific code generation utilities
pub mod struct_gen {
    use super::*;
    use proc_macro2::TokenStream;
    use quote::quote;

    /// Generates a struct definition with separate std and runtime types
    pub fn generate_struct(
        config: &GenerationConfig,
        runtime_fields: &[TokenStream],
        std_fields: &[TokenStream],
        is_unit: bool,
    ) -> TokenStream {
        use proc_macro2::Span;
        use syn::Ident;

        let type_name = &config.type_name;
        let runtime_type_name = Ident::new(&format!("Runtime{}", type_name), Span::call_site());

        let impl_generics = config.impl_generics();
        let _type_generics = config.type_generics();
        let where_clause = &config.where_clause;

        if is_unit {
            let runtime_body = quote! { struct #runtime_type_name #impl_generics #where_clause; };
            let std_body = quote! { struct #type_name #impl_generics #where_clause; };
            return super::generate_separate_types(config, runtime_body, std_body);
        }

        let runtime_body = quote! {
            struct #runtime_type_name #impl_generics #where_clause {
                #(#runtime_fields,)*
            }
        };

        let std_body = quote! {
            struct #type_name #impl_generics #where_clause {
                #(#std_fields,)*
            }
        };

        super::generate_separate_types(config, runtime_body, std_body)
    }

    /// Generates a tuple struct definition with separate std and runtime types
    pub fn generate_tuple_struct(
        config: &GenerationConfig,
        runtime_fields: &[TokenStream],
        std_fields: &[TokenStream],
    ) -> TokenStream {
        use proc_macro2::Span;
        use syn::Ident;

        let type_name = &config.type_name;
        let runtime_type_name = Ident::new(&format!("Runtime{}", type_name), Span::call_site());

        let impl_generics = config.impl_generics();
        let _type_generics = config.type_generics();
        let where_clause = &config.where_clause;

        let runtime_body = quote! {
            struct #runtime_type_name #impl_generics (#(#runtime_fields),*) #where_clause;
        };

        let std_body = quote! {
            struct #type_name #impl_generics (#(#std_fields),*) #where_clause;
        };

        super::generate_separate_types(config, runtime_body, std_body)
    }
}

/// Enum-specific code generation utilities
pub mod enum_gen {
    use super::*;
    use proc_macro2::TokenStream;
    use quote::quote;

    /// Generates an enum definition with separate std and runtime types if needed
    pub fn generate_enum(
        config: &GenerationConfig,
        runtime_variants: &[TokenStream],
        std_variants: &[TokenStream],
        has_transformations: bool,
    ) -> TokenStream {
        use proc_macro2::Span;
        use syn::Ident;

        let type_name = &config.type_name;
        let runtime_type_name = Ident::new(&format!("Runtime{}", type_name), Span::call_site());

        let impl_generics = config.impl_generics();
        let _type_generics = config.type_generics();
        let where_clause = &config.where_clause;

        if has_transformations {
            let runtime_body = quote! {
                enum #runtime_type_name #impl_generics #where_clause {
                    #(#runtime_variants),*
                }
            };

            let std_body = quote! {
                enum #type_name #impl_generics #where_clause {
                    #(#std_variants),*
                }
            };

            super::generate_separate_types(config, runtime_body, std_body)
        } else {
            // Same enum for both modes, just different derives
            let enum_body = quote! {
                enum #type_name #impl_generics #where_clause {
                    #(#std_variants),*
                }
            };

            super::generate_same_type_different_derives(config, enum_body)
        }
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
    pub fn generate_unit_variant(variant_name: &Ident, attributes: &[&Attribute]) -> TokenStream {
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
    fn test_generate_std_derives() {
        let derives = generate_std_derives();
        let derives_str = derives.to_string();

        assert!(derives_str.contains("Debug"));
        assert!(derives_str.contains("Clone"));
        assert!(derives_str.contains("PartialEq"));
        assert!(derives_str.contains("Eq"));
    }

    #[test]
    fn test_generate_separate_types() {
        let config = create_test_config();
        let runtime_body = quote! { struct RuntimeTestStruct { x: u32 } };
        let std_body = quote! { struct TestStruct { x: u32 } };

        let result = generate_separate_types(&config, runtime_body, std_body);
        let result_str = result.to_string();

        assert!(result_str.contains("# [cfg (feature = \"runtime\")]"));
        assert!(result_str.contains("# [cfg (feature = \"std\")]"));
        assert!(result_str.contains("Encode")); // Runtime derive
        assert!(result_str.contains("struct TestStruct")); // Std type
        assert!(result_str.contains("struct RuntimeTestStruct")); // Runtime type in body
    }

    #[test]
    fn test_struct_generation() {
        let config = create_test_config();
        let runtime_fields = vec![quote! { pub x: u32 }];
        let std_fields = vec![quote! { pub x: u32 }];

        let result = struct_gen::generate_struct(&config, &runtime_fields, &std_fields, false);
        let result_str = result.to_string();

        assert!(result_str.contains("struct TestStruct"));
        assert!(result_str.contains("pub x : u32"));
    }

    #[test]
    fn test_tuple_struct_generation() {
        let config = create_test_config();
        let runtime_fields = vec![quote! { u32 }];
        let std_fields = vec![quote! { u32 }];

        let result = struct_gen::generate_tuple_struct(&config, &runtime_fields, &std_fields);
        let result_str = result.to_string();

        assert!(result_str.contains("struct TestStruct ("));
        assert!(result_str.contains("u32"));
    }
}

