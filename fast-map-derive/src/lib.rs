#![recursion_limit = "128"]
#![type_length_limit="1880989"]

extern crate proc_macro;

#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use std::convert::TryFrom;

#[proc_macro_derive(FastMap, attributes(fast_map))]
pub fn fastmap_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    fastmap_derive_inner(input).unwrap()
}

fn fastmap_derive_inner(input: syn::DeriveInput) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let name = &input.ident;
    
    let mut key_type = None;
    let mut out_type = None;

    if let syn::Data::Struct(ref st) = input.data {
        if let syn::Fields::Unnamed(ref fields) = st.fields {
            if fields.unnamed.len() == 1 {
                let field = fields.unnamed.first().unwrap();
                
                if let syn::Type::Path(ref ty_path) = field.ty {
                    if let Some(ref last) = ty_path.path.segments.last() {
                        if let syn::PathArguments::AngleBracketed(ref ab) = last.arguments {
                            match ab.args.iter().collect::<Vec<_>>().as_slice() {
                                &[key, out] => {
                                    key_type = Some(key.clone());
                                    out_type = Some(out.clone());
                                },
                                _ => {}
                            }
                        }
                    }                    
                }
            }
        } 
    }

    let key_type = &key_type;
    let out_type = &out_type;

    if key_type.is_none() || out_type.is_none() {
        return Err("`FastMap` can only be derived on a `TupX` wrapping struct".into());
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let out = quote!();

    let fast_map_path: syn::Path = parse_quote!(fast_map);
    let keys_path: syn::Path = parse_quote!(keys);

    let keys = input
        .attrs
        .iter()
        .flat_map(|x| x.parse_meta())
        .filter(|x| x.path() == &fast_map_path)
        .filter_map(|x| match x {
            syn::Meta::List(ml) => Some(ml),
            _ => None,
        })
        .flat_map(|x| x.nested)
        .filter_map(|x| match x {
            syn::NestedMeta::Meta(m) => Some(m),
            _ => None,
        })
        .filter_map(|x| match x {
            syn::Meta::List(ml) => Some(ml),
            _ => None,
        })
        .filter(|x| &x.path == &keys_path)
        .flat_map(|x| x.nested)
        // .filter_map(|x| match x {
        //     syn::NestedMeta::Meta(m) => Some(m),
        //     _ => None,
        // })
        // .filter_map(|x| match x {
        //     syn::Meta::Path(m) => Some(m),
        //     _ => None,
        // })
        .collect::<Vec<_>>();

    let get_cases = keys.iter().enumerate()
        .map(|(idx, k)| {
            let idx = syn::Index::from(idx);
            quote!(&#k => self.0.tup.#idx.as_ref())
        })
        .collect::<Vec<_>>();

    let insert_cases = keys.iter().enumerate()
        .map(|(idx, k)| {
            let idx = syn::Index::from(idx);
            quote!(&#k => {
                std::mem::swap(&mut self.0.tup.#idx, &mut val);
                return val;
            })
        })
        .collect::<Vec<_>>();

    let out = quote! {

        impl #impl_generics #name #ty_generics #where_clause {

            pub fn get<T: std::borrow::Borrow<#key_type>>(&self, key: T) -> Option<&#out_type> {
                match key.borrow() {
                    #(#get_cases,)*
                    _ => None,
                }
            }

            pub fn insert<T: std::borrow::Borrow<#key_type>>(&mut self, key: T, val: #out_type) -> Option<#out_type> {
                let mut val = Some(val);
                match key.borrow() {
                    #(#insert_cases,)*
                    _ => None,
                }
            }
        }

    };


    Ok(out.into())

}