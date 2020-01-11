//! Transform an arbitrary structs to a http query params
//!
//! This crate generate a function for serialize the fields of an arbitrary structs
//! into a http query params `String` by the usage of a procedural macro with custom derive.
//! The query params `String` return has for purpose to be use with any rust client http lib.    
//!
//! # Getting Start
//!
//! Add `query_params` as a dependency to you `Cargo.toml`.
//!
//! ## Overview
//!
//! ```ignore,
//! #[macro_use]
//! extern crate query_params;
//! extern crate query_params_trait;
//!
//! use query_params_trait::QueryParams;
//!
//! #[derive(QueryParams)]
//! struct PullRequestsParametersApi {
//!     page: i32,
//!     sort: bool,
//!     direction: String,
//!     state: Vec<String>,
//!     // .. other interesting fields ..
//! }
//!
//! let pr = PullRequestsParametersApi {
//!     page: 2,
//!     sort: true,
//!     direction: "asc".to_string(),
//!     state: vec!["open".to_string(), "closed".to_string()],
//! };
//!
//! pr.query_params();
//! ```
//!
//! ## What that generate
//!
//!
//! ```ignore,
//! #[derive(QueryParams)]
//! struct PullRequestsParametersApi {
//!     page: i32,
//!     sort: bool,
//!     direction: String,
//!     state: Vec<String>,
//!     // .. other interesting fields ..
//! }
//!
//! // Code generate
//! impl PullRequestsParametersApi {
//!     fn query_params(&self) -> String {
//!         let mut buf = String::from("?");
//!         
//!         // Stuff to fill buf with the struct fields content
//!         
//!         return buf
//!     }
//!     // expect "?page=2&sort=true&direction=asc&state=open,closed" with the example above
//! }  
//! ```

#![crate_type = "proc-macro"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

use syn::{spanned::Spanned, Error, Result};

#[proc_macro_derive(QueryParams, attributes(query))]
pub fn derive_query_params(input: TokenStream) -> TokenStream {
    match derive_query_params_impl(input) {
        Ok(v) => v,
        Err(e) => TokenStream::from(e.to_compile_error()),
    }
}

fn derive_query_params_impl(input: TokenStream) -> Result<TokenStream> {
    let ast: syn::DeriveInput = syn::parse(input)?;

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let query_params = gen_serialization_query_params(&ast)?;
    let gen = quote! {
        impl #impl_generics query_params_trait::QueryParams for #name #ty_generics #where_clause {
            fn query_params(&self) -> String {
                #query_params
            }
        }
    };
    Ok(gen.into())
}

fn gen_serialization_query_params(ast: &syn::DeriveInput) -> Result<syn::export::TokenStream2> {
    match &ast.data {
        syn::Data::Struct(data) => {
            let query_params = get_print_fields(&data.fields)?;
            if query_params.len() == 0 {
                Ok(quote! {
                    String::default()
                })
            } else {
                Ok(quote! {
                    let mut buf = String::new();

                    (#(#query_params),*);

                    let len_query_params = buf.len();
                    buf.truncate(len_query_params - 1); // remove trailing ampersand

                    buf
                })
            }
        }
        _ => Err(Error::new(ast.span(), "Not a struct")),
    }
}

/// something cool
fn get_print_fields(fields: &syn::Fields) -> Result<Vec<syn::export::TokenStream2>> {
    fields
        .iter()
        .map(|field| {
            let path = if let syn::Type::Path(ref typath) = &field.ty {
                path_join(&typath.path)
            } else {
                return Err(Error::new(field.span(), "Type not handled"));
            };
            let ident = &field.ident;
            let mut name = ident.as_ref().unwrap().clone();
            for attr in &field.attrs {
                if let syn::Meta::List(list) = attr.parse_meta().unwrap() {
                    for nested in &list.nested {
                        if let syn::NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue {
                            path,
                            lit: syn::Lit::Str(lit_str),
                            ..
                        })) = nested
                        {
                            match path_join(&path).as_str() {
                                "rename" => {
                                    name = quote::format_ident!("{}", lit_str.value());
                                }
                                path => {
                                    return Err(Error::new(
                                        nested.span(),
                                        format!("Unrecognized attribute: {}", path),
                                    ))
                                }
                            }
                        } else {
                            return Err(Error::new(nested.span(), "Unrecognized nested attribute"));
                        }
                    }
                } else {
                    return Err(Error::new(attr.span(), "Unrecognized attribute"));
                }
            }
            Ok(match path.as_str() {
                "Vec" => vec_to_query_params(name, ident),
                "Option" => option_to_query_params(name, ident),
                _ => primitive_to_query_params(name, ident),
            })
        })
        .collect::<Result<_>>()
}

/// Creates a string from type path.
fn path_join(path: &syn::Path) -> String {
    segments_join(path.segments.iter())
}

/// Joins iterator of path segments into a string.
fn segments_join<'a, I: Iterator<Item = &'a syn::PathSegment>>(segments: I) -> String {
    segments
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

fn vec_to_query_params(name: syn::Ident, ident: &Option<syn::Ident>) -> syn::export::TokenStream2 {
    quote! {
        buf.push_str((format!("{}={}&",
            stringify!(#name),
            self.#ident
                .iter()
                .fold(String::new(), |acc, ref val| acc + &val.to_string() + ","))
                .as_str()
        )
        .replace(",&", "&") // remove trailing comma insert by fold
        .as_str())
    }
}

fn option_to_query_params(
    name: syn::Ident,
    ident: &Option<syn::Ident>,
) -> syn::export::TokenStream2 {
    quote! {
        if self.#ident.is_some() {
            buf.push_str(format!("{}={}&", stringify!(#name), self.#ident.as_ref().unwrap()).as_str())
        }
    }
}

fn primitive_to_query_params(
    name: syn::Ident,
    ident: &Option<syn::Ident>,
) -> syn::export::TokenStream2 {
    quote! {
        buf.push_str(format!("{}={}&", stringify!(#name), self.#ident).as_str())
    }
}
