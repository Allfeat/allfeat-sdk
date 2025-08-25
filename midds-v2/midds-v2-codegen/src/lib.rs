// This file is part of Allfeat.

// Copyright (C) 2022-2025 Allfeat.
// SPDX-License-Identifier: GPL-3.0-or-later

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Procedural macros for MIDDS v2 code generation.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use serde::Deserialize;
use std::fs;
use syn::{parse_macro_input, ItemMod, Lit, Meta};

/// Structure representing the music genres JSON file
#[derive(Deserialize, Debug)]
struct GenreData {
    genres: Vec<Genre>,
}

#[derive(Deserialize, Debug, Clone)]
struct Genre {
    id: String,
    subgenres: Option<Vec<SubGenre>>,
}

#[derive(Deserialize, Debug, Clone)]
struct SubGenre {
    id: String,
}

/// Procedural macro to generate music genres enum from JSON file
///
/// Usage:
/// ```rust
/// #[midds::music_genres(path = "./music-genres.json")]
/// pub mod genres;
/// ```
#[proc_macro_attribute]
pub fn music_genres(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemMod);

    // Parse the path argument
    let path = parse_path_from_args(args).unwrap_or_else(|err| {
        panic!("music_genres macro error: {}", err);
    });

    // Load and parse the JSON file
    let genre_data = load_genre_data(&path).unwrap_or_else(|err| {
        panic!("Failed to load genre data from '{}': {}", path, err);
    });

    // Generate the enum
    let generated_enum = generate_genre_enum(&genre_data);

    // Get the module's visibility, name, and attributes
    let vis = &input.vis;
    let mod_name = &input.ident;
    let attrs = &input.attrs;

    // Return the module with generated content inside
    let expanded = quote! {
        #(#attrs)*
        #vis mod #mod_name {
            #generated_enum
        }
    };

    TokenStream::from(expanded)
}

fn parse_path_from_args(args: TokenStream) -> Result<String, String> {
    if args.is_empty() {
        return Err("path argument is required".to_string());
    }

    let args_parsed =
        syn::parse::<Meta>(args).map_err(|e| format!("Failed to parse arguments: {}", e))?;

    match args_parsed {
        Meta::NameValue(nv) if nv.path.is_ident("path") => match nv.value {
            syn::Expr::Lit(syn::ExprLit {
                lit: Lit::Str(lit_str),
                ..
            }) => Ok(lit_str.value()),
            _ => Err("path must be a string literal".to_string()),
        },
        _ => Err("Expected 'path = \"...\"' argument".to_string()),
    }
}

fn load_genre_data(path: &str) -> Result<GenreData, Box<dyn std::error::Error>> {
    // Try to resolve path relative to CARGO_MANIFEST_DIR first
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let full_path = std::path::Path::new(&manifest_dir).join(path);

    let final_path = if full_path.exists() {
        full_path
    } else {
        std::path::PathBuf::from(path)
    };

    let content = fs::read_to_string(&final_path)
        .map_err(|e| format!("Cannot read file {:?}: {}", final_path, e))?;
    let genre_data: GenreData =
        serde_json::from_str(&content).map_err(|e| format!("Cannot parse JSON: {}", e))?;
    Ok(genre_data)
}

fn generate_genre_enum(genre_data: &GenreData) -> proc_macro2::TokenStream {
    let mut variants = Vec::new();
    let mut discriminant = 0u16;

    // Sort genres by id for consistent ordering
    let mut sorted_genres = genre_data.genres.clone();
    sorted_genres.sort_by(|a, b| a.id.cmp(&b.id));

    for genre in sorted_genres {
        // Add the main genre using the ID as identifier
        let main_genre_ident = format_ident(&genre.id);

        variants.push(quote! {
            #main_genre_ident = #discriminant
        });
        discriminant += 1;

        // Add subgenres if they exist
        if let Some(subgenres) = &genre.subgenres {
            let mut sorted_subgenres = subgenres.clone();
            sorted_subgenres.sort_by(|a, b| a.id.cmp(&b.id));

            for subgenre in sorted_subgenres {
                let subgenre_ident = format_ident(&subgenre.id);
                variants.push(quote! {
                    #subgenre_ident = #discriminant
                });
                discriminant += 1;
            }
        }
    }

    quote! {
        use parity_scale_codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
        use scale_info::TypeInfo;

        #[cfg(feature = "std")]
        use ts_rs::TS;

        /// Flat enum containing all main genres and subgenres.
        /// This enum is used directly in the blockchain to identify any genre type.
        #[derive(
            Clone,
            Copy,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Debug,
            Encode,
            Decode,
            DecodeWithMemTracking,
            TypeInfo,
            MaxEncodedLen,
        )]
        #[cfg_attr(feature = "std", derive(TS), ts(export), ts(export_to = "shared/"))]
        #[repr(u16)]
        pub enum GenreId {
            #(#variants,)*
        }
    }
}

fn format_ident(name: &str) -> syn::Ident {
    // Convert snake_case or kebab-case to PascalCase for enum variants
    let formatted = name
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
            }
        })
        .collect::<String>();

    // Clean up any remaining special characters
    let cleaned = formatted
        .replace(" ", "")
        .replace("/", "")
        .replace("-", "")
        .replace("&", "And")
        .replace("'", "")
        .replace("‑", "")
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();

    syn::Ident::new(&cleaned, Span::call_site())
}
