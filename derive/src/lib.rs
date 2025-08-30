// SPDX-License-Identifier: MIT

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

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
            out.extend(quote! {
                impl Permissive for #name {}
            });
        }
        true
    }
}

fn handle_capability(derive: &DeriveInput, out: &mut proc_macro2::TokenStream) {
    if let Some(impl_attr) = find_attr(derive, "capability") {
        let name = &derive.ident;
        match impl_attr.parse_nested_meta(|meta| {
            match meta.path.require_ident()?.to_string().as_str() {
                "inner_access" => {
                    out.extend(quote! {
                        impl InnerAccess for #name {}
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
        match impl_attr.parse_nested_meta(|meta| {
            match meta.path.require_ident()?.to_string().as_str() {
                s @ ("Default" | "Clone" | "Copy" | "PartialEq" | "Eq" | "Hash" | "Deref") => {
                    let trait_name = quote::format_ident!("Implement{s}");
                    out.extend(quote! {
                        impl #trait_name for #name {}
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
    if let Some(impl_attr) = find_attr(derive, "transparent") {
        match impl_attr.parse_nested_meta(|meta| {
            match meta.path.require_ident()?.to_string().as_str() {
                s @ ("Display" | "Debug" | "FromStr" | "Serialize" | "Deserialize") => {
                    let trait_name = quote::format_ident!("Transparent{s}");
                    out.extend(quote! {
                        impl #trait_name for #name {}
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
