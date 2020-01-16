#![recursion_limit = "128"]
#![type_length_limit = "1880989"]

extern crate proc_macro;

#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(FastMap, attributes(fast_map))]
pub fn fastmap_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    fastmap_derive_inner(input).unwrap()
}

fn fastmap_derive_inner(
    input: syn::DeriveInput,
) -> Result<TokenStream, Box<dyn std::error::Error>> {
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
                                }
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
        return Err("`FastMap` can only be derived on a `MapX` wrapping struct".into());
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fast_map_path: syn::Path = parse_quote!(fast_map);
    let keys_path: syn::Path = parse_quote!(keys);
    let crate_name_path: syn::Path = parse_quote!(crate_name);
    let infallible_path: syn::Path = parse_quote!(infallible);

    let l_attrs = input
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
        });

    let keys = l_attrs
        .clone()
        .filter_map(|x| match x {
            syn::Meta::List(ml) => Some(ml),
            _ => None,
        })
        .filter(|x| &x.path == &keys_path)
        .flat_map(|x| x.nested)
        .collect::<Vec<_>>();

    let crate_name = l_attrs
        .clone()
        .filter_map(|x| match x {
            syn::Meta::NameValue(mnv) => Some(mnv),
            _ => None,
        })
        .filter(|x| &x.path == &crate_name_path)
        .filter_map(|x| match x.lit {
            syn::Lit::Str(s) => Some(s),
            _ => None,
        })
        .map(|x| syn::Ident::new(&x.value(), proc_macro2::Span::call_site()))
        .next()
        .unwrap_or_else(|| syn::Ident::new("fast_map", proc_macro2::Span::call_site()));

    let is_infallible = l_attrs
        .clone()
        .filter_map(|x| match x {
            syn::Meta::NameValue(mnv) => Some(mnv),
            _ => None,
        })
        .filter(|x| &x.path == &infallible_path)
        .filter_map(|x| match x.lit {
            syn::Lit::Bool(s) => Some(s.value),
            _ => None,
        })
        .next()
        .unwrap_or(false);

    let get_cases = keys
        .iter()
        .enumerate()
        .map(|(idx, k)| {
            let idx = syn::Index::from(idx);
            let ret = quote!(Ok(self.0.tup.#idx.as_ref()));
            quote!(#k => #ret)
        })
        .collect::<Vec<_>>();

    let get_mut_cases = keys
        .iter()
        .enumerate()
        .map(|(idx, k)| {
            let idx = syn::Index::from(idx);
            let ret = quote!(Ok(self.0.tup.#idx.as_mut()));
            quote!(#k => #ret)
        })
        .collect::<Vec<_>>();

    let insert_cases = keys
        .iter()
        .enumerate()
        .map(|(idx, k)| {
            let idx = syn::Index::from(idx);
            let ret = quote!(Ok(self.0.tup.#idx.replace(val)));
            quote!(#k => #ret)
        })
        .collect::<Vec<_>>();

    let remove_cases = keys
        .iter()
        .enumerate()
        .map(|(idx, k)| {
            let idx = syn::Index::from(idx);
            let ret = quote!(Ok(self.0.tup.#idx.take()));
            quote!(#k => #ret)
        })
        .collect::<Vec<_>>();

    let values = keys
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            let idx = syn::Index::from(idx);
            quote!(self.0.tup.#idx.as_ref())
        })
        .collect::<Vec<_>>();
    let out = if is_infallible {
        quote! {
            impl #impl_generics #name #ty_generics #where_clause {

                fn get<T: std::borrow::Borrow<#key_type>>(&self, key: T) -> Option<&#out_type> {
                    (match key.borrow() {
                        #(#get_cases,)*
                        _ => Err(#crate_name::Error::KeyNotFound),
                    }).unwrap()
                }

                fn get_mut<T: std::borrow::Borrow<#key_type>>(&mut self, key: T) -> Option<&mut #out_type> {
                    (match key.borrow() {
                        #(#get_mut_cases,)*
                        _ => Err(#crate_name::Error::KeyNotFound),
                    }).unwrap()
                }

                fn insert<T: std::borrow::Borrow<#key_type>>(&mut self, key: T, mut val: #out_type) -> Option<#out_type> {
                    (match key.borrow() {
                        #(#insert_cases,)*
                        _ => Err(#crate_name::Error::KeyNotFound),
                    }).unwrap()
                }

                fn remove<T: std::borrow::Borrow<#key_type>>(&mut self, key: T) -> Option<#out_type> {
                    (match key.borrow() {
                        #(#remove_cases,)*
                        _ => Err(#crate_name::Error::KeyNotFound),
                    }).unwrap()
                }

                fn values<'fast_map>(&'fast_map self) -> #crate_name::Values<'fast_map, #out_type> {
                    #crate_name::Values::new(vec![#(#values,)*].into_iter())
                }

            }
        }
    } else {
        quote! {
            impl #impl_generics #name #ty_generics #where_clause {

                fn get<T: std::borrow::Borrow<#key_type>>(&self, key: T) -> std::result::Result<Option<&#out_type>, #crate_name::Error> {
                    match key.borrow() {
                        #(#get_cases,)*
                        _ => Err(#crate_name::Error::KeyNotFound),
                    }
                }

                fn get_mut<T: std::borrow::Borrow<#key_type>>(&mut self, key: T) -> std::result::Result<Option<&mut #out_type>, #crate_name::Error> {
                    match key.borrow() {
                        #(#get_mut_cases,)*
                        _ => Err(#crate_name::Error::KeyNotFound),
                    }
                }

                fn insert<T: std::borrow::Borrow<#key_type>>(&mut self, key: T, val: #out_type) -> std::result::Result<Option<#out_type>, #crate_name::Error> {
                    let mut val = val;
                    match key.borrow() {
                        #(#insert_cases,)*
                        _ => Err(#crate_name::Error::KeyNotFound),
                    }
                }

                fn remove<T: std::borrow::Borrow<#key_type>>(&mut self, key: T) -> std::result::Result<Option<#out_type>, #crate_name::Error> {
                    match key.borrow() {
                        #(#remove_cases,)*
                        _ => Err(#crate_name::Error::KeyNotFound),
                    }
                }

                fn values<'fast_map>(&'fast_map self) -> #crate_name::Values<'fast_map, #out_type> {
                    #crate_name::Values::new(vec![#(#values,)*].into_iter())
                }

            }
        }
    };

    Ok(out.into())
}
