//! Attribute parsing utilities for the MIDDS V2 procedural macro.
//!
//! This module handles parsing and validation of attributes:
//! - `#[runtime_bound(N)]` - specify maximum sizes for bounded types in runtime mode
//! - `#[as_runtime_type]` - transform type to Runtime{Type} equivalent in runtime mode

use proc_macro2::TokenStream;
use syn::{parse::Parser, Attribute, Expr, Lit, LitInt, Meta};

use crate::error::{MacroError, MacroResult};

/// Represents a parsed runtime bound attribute with validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeBound {
    /// The primary bound value (for single values or vec size for nested bounds)
    pub value: u32,
    /// Optional secondary bound value (for string size in Vec<String>)
    pub secondary_value: Option<u32>,
    /// The original literal token(s) for code generation
    pub literal: LitInt,
    /// Optional secondary literal for nested bounds
    pub secondary_literal: Option<LitInt>,
}

impl RuntimeBound {
    /// Creates a new RuntimeBound from a single parsed integer literal
    pub fn new(literal: LitInt) -> MacroResult<Self> {
        let value = literal
            .base10_parse::<u32>()
            .map_err(|_| MacroError::invalid_bound_syntax(&literal, "bound must be a valid u32"))?;

        if value == 0 {
            return Err(MacroError::invalid_bound_syntax(
                &literal,
                "bound must be greater than 0",
            ));
        }

        Ok(Self {
            value,
            secondary_value: None,
            literal,
            secondary_literal: None,
        })
    }

    /// Creates a new RuntimeBound with two values (vec_size, string_size)
    pub fn new_nested(primary: LitInt, secondary: LitInt) -> MacroResult<Self> {
        let value = primary.base10_parse::<u32>().map_err(|_| {
            MacroError::invalid_bound_syntax(&primary, "primary bound must be a valid u32")
        })?;

        let secondary_value = secondary.base10_parse::<u32>().map_err(|_| {
            MacroError::invalid_bound_syntax(&secondary, "secondary bound must be a valid u32")
        })?;

        if value == 0 {
            return Err(MacroError::invalid_bound_syntax(
                &primary,
                "primary bound must be greater than 0",
            ));
        }

        if secondary_value == 0 {
            return Err(MacroError::invalid_bound_syntax(
                &secondary,
                "secondary bound must be greater than 0",
            ));
        }

        Ok(Self {
            value,
            secondary_value: Some(secondary_value),
            literal: primary,
            secondary_literal: Some(secondary),
        })
    }

    /// Gets the primary bound value as a u32
    #[allow(dead_code)]
    pub fn value(&self) -> u32 {
        self.value
    }

    /// Checks if this is a nested bound (has two values)
    pub fn is_nested(&self) -> bool {
        self.secondary_value.is_some()
    }

    /// Gets the primary literal token for code generation
    pub fn literal(&self) -> &LitInt {
        &self.literal
    }

    /// Gets the secondary literal token for code generation if it exists
    pub fn secondary_literal(&self) -> Option<&LitInt> {
        self.secondary_literal.as_ref()
    }

    /// Converts to a token stream for code generation (primary value only)
    #[allow(dead_code)]
    pub fn to_tokens(&self) -> TokenStream {
        let lit = &self.literal;
        quote::quote! { #lit }
    }
}

/// Represents an as_runtime_type annotation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsRuntimeType {
    /// Optional path to the runtime type (e.g., "iswc" for iswc::RuntimeIswc)
    pub path: Option<String>,
}

/// Attribute parser for extracting runtime bounds and as_runtime_type from attributes
pub struct AttributeParser;

impl AttributeParser {
    /// Extracts the runtime bound from an attribute list
    ///
    /// Returns `None` if no runtime_bound attribute is found,
    /// `Some(Ok(bound))` if a valid bound is found,
    /// `Some(Err(error))` if an invalid bound is found.
    pub fn extract_runtime_bound(attrs: &[Attribute]) -> Option<MacroResult<RuntimeBound>> {
        attrs.iter().find_map(|attr| {
            if !attr.path().is_ident("runtime_bound") {
                return None;
            }

            let bound_result = Self::parse_runtime_bound_attr(attr);
            Some(bound_result)
        })
    }

    /// Extracts the as_runtime_type annotation from an attribute list
    ///
    /// Returns `Some(AsRuntimeType)` if the attribute is found,
    /// `None` if not found.
    pub fn extract_as_runtime_type(attrs: &[Attribute]) -> Option<AsRuntimeType> {
        attrs.iter().find_map(|attr| {
            if attr.path().is_ident("as_runtime_type") {
                let path = Self::parse_as_runtime_type_path(attr).ok().flatten();
                Some(AsRuntimeType { path })
            } else {
                None
            }
        })
    }

    /// Parses the optional path parameter from as_runtime_type attribute
    ///
    /// Supports: `#[as_runtime_type]` and `#[as_runtime_type(path = "module")]`
    fn parse_as_runtime_type_path(attr: &Attribute) -> MacroResult<Option<String>> {
        match &attr.meta {
            // #[as_runtime_type] - no parameters
            Meta::Path(_) => Ok(None),

            // #[as_runtime_type(path = "module")] - with parameters
            Meta::List(meta_list) => {
                let expr: Expr = syn::parse2(meta_list.tokens.clone()).map_err(|_| {
                    MacroError::invalid_bound_syntax(attr, "expected path = \"module\" format")
                })?;

                // Parse "path = \"value\""
                if let Expr::Assign(assign) = expr
                    && let (Expr::Path(path_expr), Expr::Lit(lit_expr)) =
                        (&*assign.left, &*assign.right)
                    && path_expr.path.is_ident("path")
                    && let Lit::Str(lit_str) = &lit_expr.lit
                {
                    return Ok(Some(lit_str.value()));
                }

                Err(MacroError::invalid_bound_syntax(
                    attr,
                    "expected path = \"module\" format",
                ))
            }

            _ => Err(MacroError::invalid_bound_syntax(
                attr,
                "expected #[as_runtime_type] or #[as_runtime_type(path = \"module\")] format",
            )),
        }
    }

    /// Parses a runtime_bound attribute - supports both single and double values
    /// #[runtime_bound(N)] or #[runtime_bound(vec_size, string_size)]
    fn parse_runtime_bound_attr(attr: &Attribute) -> MacroResult<RuntimeBound> {
        let Meta::List(meta_list) = &attr.meta else {
            return Err(MacroError::invalid_bound_syntax(
                attr,
                "expected #[runtime_bound(N)] or #[runtime_bound(vec_size, string_size)] format",
            ));
        };

        // Try to parse as multiple expressions separated by commas
        let parsed: syn::punctuated::Punctuated<Expr, syn::Token![,]> = {
            let parser = syn::punctuated::Punctuated::<Expr, syn::Token![,]>::parse_terminated;
            parser.parse2(meta_list.tokens.clone()).map_err(|_| {
                MacroError::invalid_bound_syntax(attr, "expected numeric literal(s)")
            })?
        };

        match parsed.len() {
            1 => {
                // Single value: #[runtime_bound(N)]
                let expr = &parsed[0];
                let Expr::Lit(expr_lit) = expr else {
                    return Err(MacroError::invalid_bound_syntax(
                        attr,
                        "expected a numeric literal",
                    ));
                };

                let Lit::Int(lit_int) = &expr_lit.lit else {
                    return Err(MacroError::invalid_bound_syntax(
                        attr,
                        "expected an integer literal",
                    ));
                };

                RuntimeBound::new(lit_int.clone())
            }
            2 => {
                // Double value: #[runtime_bound(vec_size, string_size)]
                let first_expr = &parsed[0];
                let second_expr = &parsed[1];

                let Expr::Lit(first_lit) = first_expr else {
                    return Err(MacroError::invalid_bound_syntax(
                        attr,
                        "expected first numeric literal",
                    ));
                };

                let Expr::Lit(second_lit) = second_expr else {
                    return Err(MacroError::invalid_bound_syntax(
                        attr,
                        "expected second numeric literal",
                    ));
                };

                let Lit::Int(first_int) = &first_lit.lit else {
                    return Err(MacroError::invalid_bound_syntax(
                        attr,
                        "expected first integer literal",
                    ));
                };

                let Lit::Int(second_int) = &second_lit.lit else {
                    return Err(MacroError::invalid_bound_syntax(
                        attr,
                        "expected second integer literal",
                    ));
                };

                RuntimeBound::new_nested(first_int.clone(), second_int.clone())
            }
            _ => Err(MacroError::invalid_bound_syntax(
                attr,
                "expected 1 or 2 numeric literals: #[runtime_bound(N)] or #[runtime_bound(vec_size, string_size)]",
            ))
        }
    }

    /// Filters out runtime_bound and as_runtime_type attributes from an attribute list
    ///
    /// This is used when generating the final code to avoid including
    /// the macro-specific attributes in the output.
    pub fn filter_runtime_bound_attrs(attrs: &[Attribute]) -> Vec<&Attribute> {
        attrs
            .iter()
            .filter(|attr| {
                !attr.path().is_ident("runtime_bound") && !attr.path().is_ident("as_runtime_type")
            })
            .collect()
    }

    /// Validates that an attribute list doesn't contain conflicting attributes
    pub fn validate_attributes(attrs: &[Attribute]) -> MacroResult<()> {
        let runtime_bound_count = attrs
            .iter()
            .filter(|attr| attr.path().is_ident("runtime_bound"))
            .count();

        if runtime_bound_count > 1 {
            // Find the first runtime_bound attribute for error reporting
            let first_bound_attr = attrs
                .iter()
                .find(|attr| attr.path().is_ident("runtime_bound"))
                .unwrap();

            return Err(MacroError::conflicting_attributes(
                first_bound_attr,
                "multiple #[runtime_bound(N)] attributes found on the same item",
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_runtime_bound_creation() {
        let lit: LitInt = syn::parse_str("42").unwrap();
        let bound = RuntimeBound::new(lit).unwrap();

        assert_eq!(bound.value(), 42);
    }

    #[test]
    fn test_runtime_bound_zero_value() {
        let lit: LitInt = syn::parse_str("0").unwrap();
        let result = RuntimeBound::new(lit);

        assert!(result.is_err());
    }

    #[test]
    fn test_extract_runtime_bound_present() {
        let attrs: Vec<Attribute> = vec![
            parse_quote! { #[runtime_bound(128)] },
            parse_quote! { #[derive(Debug)] },
        ];

        let result = AttributeParser::extract_runtime_bound(&attrs);
        assert!(result.is_some());

        let bound = result.unwrap().unwrap();
        assert_eq!(bound.value(), 128);
    }

    #[test]
    fn test_extract_runtime_bound_missing() {
        let attrs: Vec<Attribute> = vec![
            parse_quote! { #[derive(Debug)] },
            parse_quote! { #[serde(rename = "test")] },
        ];

        let result = AttributeParser::extract_runtime_bound(&attrs);
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_runtime_bound_invalid() {
        let attrs: Vec<Attribute> = vec![parse_quote! { #[runtime_bound("invalid")] }];

        let result = AttributeParser::extract_runtime_bound(&attrs);
        assert!(result.is_some());
        assert!(result.unwrap().is_err());
    }

    #[test]
    fn test_filter_runtime_bound_attrs() {
        let attrs: Vec<Attribute> = vec![
            parse_quote! { #[runtime_bound(128)] },
            parse_quote! { #[derive(Debug)] },
            parse_quote! { #[serde(rename = "test")] },
        ];

        let filtered = AttributeParser::filter_runtime_bound_attrs(&attrs);
        assert_eq!(filtered.len(), 2);

        // Should not contain runtime_bound
        assert!(!filtered
            .iter()
            .any(|attr| attr.path().is_ident("runtime_bound")));
    }

    #[test]
    fn test_validate_attributes_single_bound() {
        let attrs: Vec<Attribute> = vec![
            parse_quote! { #[runtime_bound(128)] },
            parse_quote! { #[derive(Debug)] },
        ];

        let result = AttributeParser::validate_attributes(&attrs);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_attributes_multiple_bounds() {
        let attrs: Vec<Attribute> = vec![
            parse_quote! { #[runtime_bound(128)] },
            parse_quote! { #[runtime_bound(256)] },
        ];

        let result = AttributeParser::validate_attributes(&attrs);
        assert!(result.is_err());
    }
}
