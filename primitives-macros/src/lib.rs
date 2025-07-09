use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, ItemStruct, parse_macro_input};

#[proc_macro_attribute]
pub fn allfeat_string(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name: Ident = input.ident.clone();

    let expanded = quote! {
        #[::wasm_bindgen::prelude::wasm_bindgen]
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct #name(String);

        impl ::core::fmt::Display for #name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl ::core::ops::Deref for #name {
            type Target = String;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<String> for #name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<#name> for String {
            fn from(value: #name) -> Self {
                value.0
            }
        }

        impl ::core::convert::TryFrom<::std::vec::Vec<u8>> for #name {
            type Error = ::wasm_bindgen::JsError;

            fn try_from(value: ::std::vec::Vec<u8>) -> ::core::result::Result<Self, Self::Error> {
                let string = String::from_utf8(value)
                    .map_err(|e| ::wasm_bindgen::JsError::new(&format!("Couldn't decode UTF-8: {e}")))?;
                Ok(Self(string))
            }
        }

        impl ::core::convert::TryFrom<&melodie::runtime_types::bounded_collections::bounded_vec::BoundedVec<u8>> for #name {
            type Error = ::wasm_bindgen::JsError;

            fn try_from(value: &melodie::runtime_types::bounded_collections::bounded_vec::BoundedVec<u8>) -> ::core::result::Result<Self, Self::Error> {
                let string = String::from_utf8(value.0.clone())
                    .map_err(|e| ::wasm_bindgen::JsError::new(&format!("Couldn't decode UTF-8 in BoundedVec: {e}")))?;
                Ok(Self(string))
            }
        }

        impl ::core::convert::From<#name> for melodie::runtime_types::bounded_collections::bounded_vec::BoundedVec<u8> {
            fn from(value: #name) -> Self {
                melodie::runtime_types::bounded_collections::bounded_vec::BoundedVec(value.0.into_bytes())
            }
        }

        impl From<#name> for ::std::vec::Vec<u8> {
            fn from(value: #name) -> Self {
                value.0.into_bytes()
            }
        }

        #[::wasm_bindgen::prelude::wasm_bindgen]
        impl #name {
            #[::wasm_bindgen::prelude::wasm_bindgen(constructor)]
            pub fn new(value: String) -> Self {
                Self(value)
            }

            #[::wasm_bindgen::prelude::wasm_bindgen(js_name = "toString")]
            pub fn to_string_js(&self) -> String {
                self.0.clone()
            }
        }
    };

    expanded.into()
}
