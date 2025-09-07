// SPDX-License-Identifier: MIT

#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

//! tagged-types-derive provides derive macro for a Tag types
//!
//! To enable impementions on `TaggedType` it expects implementation
//! of traits for Tag type. This crates give possibility of implementation
//! of the traits using derive syntax.
//!
//! Example with all defined attributes:
//! ```rust
//! use tagged_types::TaggedType;
//! type Host = TaggedType<String, HostTag>;
//! #[derive(tagged_types_derive::Tag)]
//! #[implement(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
//! #[transparent(Debug, Display, FromStr)]
//! #[capability(inner_access, from_inner)]
//! enum HostTag {}
//!
//! let host = Host::default();
//! ```
//!
//! Alternative, using permissive attribute:
//! ```rust
//! use tagged_types::TaggedType;
//! type Host = TaggedType<String, HostTag>;
//! #[derive(tagged_types_derive::Tag)]
//! #[permissive]
//! enum HostTag {}
//!
//! let host = Host::default();
//! ```

#![deny(missing_docs)]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::DeriveInput;

/// # Attributes
///
/// - `#[implement(...)]`\
///   List **individual traits** to implement.
///   Supported:
///    - `Default`
///    - `Clone`
///    - `Copy`
///    - `PartialEq`
///    - `Eq`
///    - `PartialOrd`
///    - `Ord`
///    - `Hash`
///    - `Add`
///    - `Sub`
///    - `Mul`
///    - `Div`
///
/// - `#[transparent]`\
///   Transparent implementations as if no wrapper at all.
///   Supported:
///    - `Display`
///    - `Debug`
///    - `FromStr`
///
/// - `#[capability(...)]`\
///   Enable additional capabilities for `TaggedType`.
///   Supported:
///   - `inner_access` provides `into_inner()` and `inner()` functions.
///   - `from_inner` provides implmentation `From<Inner>` for `TaggedType<Inner, Tag>`.
///   - `value_map` provides `map(self, F)` and `try_map(self, F)` for `TaggedType<Inner, Tag>`.
///   - `cloned` provides `cloned(self)` for `TaggedType<&Inner, Tag>`.
///
/// - `#[permissive]`\
///   Convenience mode that implents all supported capabilities, implentations and transparent
///   implementations of traits.
#[proc_macro_derive(Tag, attributes(implement, transparent, capability, permissive))]
pub fn derive_tag(input: TokenStream) -> TokenStream {
    let derive = syn::parse_macro_input!(input as syn::DeriveInput);
    let mut out = quote! {};
    if !handle_permissive(&derive, &mut out) {
        handle_capability(&derive, &mut out);
        handle_implement(&derive, &mut out);
        handle_transparent(&derive, &mut out);
    }
    TokenStream::from(out)
}

fn find_attr<'a>(derive: &'a DeriveInput, attr_name: &str) -> Option<&'a syn::Attribute> {
    derive
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident(attr_name))
}

fn handle_permissive(derive: &DeriveInput, out: &mut proc_macro2::TokenStream) -> bool {
    if find_attr(derive, "permissive").is_none() {
        false
    } else {
        if derive.attrs.len() > 1 {
            out.extend(quote! {
                compile_error!("permissive must be the only attribute in derive");
            });
        } else {
            let name = &derive.ident;
            let tt = crate_path();
            out.extend(quote! {
                impl #tt::Permissive for #name {}
            });
        }
        true
    }
}

fn handle_capability(derive: &DeriveInput, out: &mut proc_macro2::TokenStream) {
    if let Some(impl_attr) = find_attr(derive, "capability") {
        let name = &derive.ident;
        let tt = crate_path();
        match impl_attr.parse_nested_meta(|meta| {
            match meta.path.require_ident()?.to_string().as_str() {
                "inner_access" => {
                    out.extend(quote! {
                        impl #tt::InnerAccess for #name {}
                    });
                    Ok(())
                }
                "from_inner" => {
                    out.extend(quote! {
                        impl #tt::FromInner for #name {}
                    });
                    Ok(())
                }
                "value_map" => {
                    out.extend(quote! {
                        impl #tt::ValueMap for #name {}
                    });
                    Ok(())
                }
                "cloned" => {
                    out.extend(quote! {
                        impl #tt::Cloned for #name {}
                    });
                    Ok(())
                }
                "as_ref" => {
                    out.extend(quote! {
                        impl #tt::AsRef for #name {}
                    });
                    Ok(())
                }
                v => Err(meta.error(format!("Don't know capability: {v}"))),
            }
        }) {
            Ok(()) => (),
            Err(e) => out.extend(e.into_compile_error()),
        }
    }
}

fn handle_implement(derive: &DeriveInput, out: &mut proc_macro2::TokenStream) {
    if let Some(impl_attr) = find_attr(derive, "implement") {
        let name = &derive.ident;
        let tt = crate_path();
        match impl_attr.parse_nested_meta(|meta| {
            match meta.path.require_ident()?.to_string().as_str() {
                s @ ("Default" | "Clone" | "Copy" | "PartialEq" | "Eq" | "PartialOrd" | "Ord"
                | "Hash" | "Deref" | "Add" | "Sub" | "Mul" | "Div") => {
                    let trait_name = quote::format_ident!("Implement{s}");
                    out.extend(quote! {
                        impl #tt::#trait_name for #name {}
                    });
                    Ok(())
                }
                v => Err(meta.error(format!("Don't know how to implement: {v}"))),
            }
        }) {
            Ok(()) => (),
            Err(e) => out.extend(e.into_compile_error()),
        }
    }
}

fn handle_transparent(derive: &DeriveInput, out: &mut proc_macro2::TokenStream) {
    let name = &derive.ident;
    let tt = crate_path();
    if let Some(impl_attr) = find_attr(derive, "transparent") {
        match impl_attr.parse_nested_meta(|meta| {
            match meta.path.require_ident()?.to_string().as_str() {
                s @ ("Display" | "Debug" | "FromStr" | "Serialize" | "Deserialize") => {
                    let trait_name = quote::format_ident!("Transparent{s}");
                    out.extend(quote! {
                        impl #tt::#trait_name for #name {}
                    });
                    Ok(())
                }
                v => Err(meta.error(format!("Don't know how to make {v} transparent"))),
            }
        }) {
            Ok(()) => (),
            Err(e) => out.extend(e.into_compile_error()),
        }
    }
}

fn crate_path() -> syn::Path {
    use proc_macro_crate::{crate_name, FoundCrate};
    match crate_name("tagged-types") {
        // The macro is used *inside* the tagged-types crate
        Ok(FoundCrate::Itself) => syn::parse_quote!(crate),
        // The user may have renamed the dependency in Cargo.toml
        Ok(FoundCrate::Name(name)) => {
            let ident = syn::Ident::new(&name, Span::call_site());
            syn::parse_quote!(::#ident)
        }
        // Fallback (shouldn’t normally happen)
        Err(_) => syn::parse_quote!(::tagged_types),
    }
}
