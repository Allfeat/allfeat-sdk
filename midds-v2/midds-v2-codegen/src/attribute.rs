//! Attribute parsing utilities for the MIDDS V2 procedural macro.
//!
//! This module handles parsing and validation of the `#[runtime_bound(N)]` attributes
//! that specify maximum sizes for bounded types in runtime mode.

use proc_macro2::TokenStream;
use syn::{Attribute, Expr, Lit, LitInt, Meta};

use crate::error::{MacroError, MacroResult};

/// Represents a parsed runtime bound attribute with validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeBound {
    /// The bound value (maximum size)
    pub value: u32,
    /// The original literal token for code generation
    pub literal: LitInt,
}

impl RuntimeBound {
    /// Creates a new RuntimeBound from a parsed integer literal
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

        Ok(Self { value, literal })
    }

    /// Gets the bound value as a u32
    #[allow(dead_code)]
    pub fn value(&self) -> u32 {
        self.value
    }

    /// Gets the literal token for code generation
    pub fn literal(&self) -> &LitInt {
        &self.literal
    }

    /// Converts to a token stream for code generation
    #[allow(dead_code)]
    pub fn to_tokens(&self) -> TokenStream {
        let lit = &self.literal;
        quote::quote! { #lit }
    }
}

/// Attribute parser for extracting runtime bounds from attributes
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

    /// Parses a single runtime_bound attribute
    fn parse_runtime_bound_attr(attr: &Attribute) -> MacroResult<RuntimeBound> {
        let Meta::List(meta_list) = &attr.meta else {
            return Err(MacroError::invalid_bound_syntax(
                attr,
                "expected #[runtime_bound(N)] format",
            ));
        };

        let expr: Expr = syn::parse2(meta_list.tokens.clone()).map_err(|_| {
            MacroError::invalid_bound_syntax(attr, "expected a numeric literal")
        })?;

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

    /// Filters out runtime_bound attributes from an attribute list
    /// 
    /// This is used when generating the final code to avoid including
    /// the macro-specific attributes in the output.
    pub fn filter_runtime_bound_attrs(attrs: &[Attribute]) -> Vec<&Attribute> {
        attrs
            .iter()
            .filter(|attr| !attr.path().is_ident("runtime_bound"))
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
        let attrs: Vec<Attribute> = vec![
            parse_quote! { #[runtime_bound("invalid")] },
        ];

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
        assert!(!filtered.iter().any(|attr| attr.path().is_ident("runtime_bound")));
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