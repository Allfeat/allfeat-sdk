//! Error handling for the MIDDS V2 procedural macro.
//!
//! This module provides structured error handling with detailed diagnostic messages
//! to help developers understand and fix issues with their `#[runtime_midds]` usage.

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::spanned::Spanned;

/// Represents different types of errors that can occur during macro expansion.
#[derive(Debug, Clone)]
pub enum MacroError {
    /// An unsupported data structure was passed to the macro
    UnsupportedDataStructure {
        /// The span of the unsupported structure
        span: Span,
        /// Human-readable description of what was attempted
        attempted: String,
    },
    /// A field that requires a bound is missing the `#[runtime_bound(N)]` attribute
    MissingRuntimeBound {
        /// The span of the field missing the bound
        span: Span,
        /// The type of the field that needs a bound
        field_type: String,
    },
    /// The `#[runtime_bound(N)]` attribute has invalid syntax
    InvalidBoundSyntax {
        /// The span of the invalid attribute
        span: Span,
        /// Description of what was wrong with the syntax
        reason: String,
    },
    /// An unsupported type was used with `#[runtime_bound(N)]`
    UnsupportedBoundType {
        /// The span of the unsupported type
        span: Span,
        /// The type that doesn't support bounds
        type_name: String,
    },
    /// Multiple conflicting attributes were found
    ConflictingAttributes {
        /// The span of the conflicting attributes
        span: Span,
        /// Description of the conflict
        conflict: String,
    },
}

impl MacroError {
    /// Creates an error for unsupported data structures (like unions)
    pub fn unsupported_data_structure<T: Spanned>(item: &T, attempted: &str) -> Self {
        Self::UnsupportedDataStructure {
            span: item.span(),
            attempted: attempted.to_string(),
        }
    }

    /// Creates an error for fields missing required runtime bounds
    pub fn missing_runtime_bound<T: Spanned>(field: &T, field_type: &str) -> Self {
        Self::MissingRuntimeBound {
            span: field.span(),
            field_type: field_type.to_string(),
        }
    }

    /// Creates an error for invalid bound attribute syntax
    pub fn invalid_bound_syntax<T: Spanned>(attr: &T, reason: &str) -> Self {
        Self::InvalidBoundSyntax {
            span: attr.span(),
            reason: reason.to_string(),
        }
    }

    /// Creates an error for unsupported types with bounds
    pub fn unsupported_bound_type<T: Spanned>(ty: &T, type_name: &str) -> Self {
        Self::UnsupportedBoundType {
            span: ty.span(),
            type_name: type_name.to_string(),
        }
    }

    /// Creates an error for conflicting attributes
    pub fn conflicting_attributes<T: Spanned>(item: &T, conflict: &str) -> Self {
        Self::ConflictingAttributes {
            span: item.span(),
            conflict: conflict.to_string(),
        }
    }

    /// Converts the error into a compiler error token stream
    pub fn into_compile_error(self) -> TokenStream {
        match self {
            Self::UnsupportedDataStructure { span, attempted } => {
                let msg = format!(
                    "runtime_midds macro does not support {}. Only structs and enums are supported.",
                    attempted
                );
                syn::Error::new(span, msg).to_compile_error()
            }
            Self::MissingRuntimeBound { span, field_type } => {
                let msg = format!(
                    "Field of type `{}` requires a `#[runtime_bound(N)]` attribute to specify the maximum size in runtime mode. \
                     Add `#[runtime_bound(N)]` where N is the maximum bound size.",
                    field_type
                );
                syn::Error::new(span, msg).to_compile_error()
            }
            Self::InvalidBoundSyntax { span, reason } => {
                let msg = format!(
                    "Invalid `#[runtime_bound(N)]` syntax: {}. \
                     Expected format: `#[runtime_bound(123)]` where the number is a positive integer.",
                    reason
                );
                syn::Error::new(span, msg).to_compile_error()
            }
            Self::UnsupportedBoundType { span, type_name } => {
                let msg = format!(
                    "Type `{}` does not support `#[runtime_bound(N)]` attribute. \
                     Only String, Vec<T>, Option<String>, Option<Vec<T>>, and &str are supported.",
                    type_name
                );
                syn::Error::new(span, msg).to_compile_error()
            }
            Self::ConflictingAttributes { span, conflict } => {
                let msg = format!(
                    "Conflicting attributes found: {}. \
                     Please resolve the conflict by removing duplicate or incompatible attributes.",
                    conflict
                );
                syn::Error::new(span, msg).to_compile_error()
            }
        }
    }
}


/// Result type for macro operations
pub type MacroResult<T> = Result<T, MacroError>;

/// Collects multiple errors and converts them into a combined compile error
#[allow(dead_code)]
pub fn combine_errors(errors: Vec<MacroError>) -> TokenStream {
    if errors.is_empty() {
        return quote! {};
    }

    let error_tokens: Vec<_> = errors
        .into_iter()
        .map(|err| err.into_compile_error())
        .collect();

    quote! {
        #(#error_tokens)*
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Span;
    
    #[test]
    fn test_error_creation() {
        let span = Span::call_site();
        
        let error = MacroError::UnsupportedDataStructure {
            span,
            attempted: "union".to_string(),
        };
        
        let tokens = error.into_compile_error();
        let tokens_str = tokens.to_string();
        
        assert!(tokens_str.contains("compile_error"));
        assert!(tokens_str.contains("union"));
    }
    
    #[test]
    fn test_combine_errors() {
        let errors = vec![
            MacroError::UnsupportedDataStructure {
                span: Span::call_site(),
                attempted: "union".to_string(),
            },
            MacroError::MissingRuntimeBound {
                span: Span::call_site(),
                field_type: "String".to_string(),
            },
        ];
        
        let combined = combine_errors(errors);
        let combined_str = combined.to_string();
        
        assert!(combined_str.contains("compile_error"));
    }
}