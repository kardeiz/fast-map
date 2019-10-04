#[macro_use]
extern crate quote;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("tups.inner.rs");

    let out = (1..=32).into_iter().map(|i| {
        let tup_ty = (0..i).into_iter().map(|_| quote!(Option<T>)).collect::<Vec<_>>();
        let nones = (0..i).into_iter().map(|_| quote!(None)).collect::<Vec<_>>();
        let name = format_ident!("Tup{}", i as u32);

        quote! {
            pub struct #name<K: ?Sized, T> {
                pub tup: (#(#tup_ty,)*),
                key: std::marker::PhantomData<dyn Fn() -> K>
            }


            impl<K: ?Sized, T> Default for #name<K, T> {
                fn default() -> Self {
                    #name {
                        tup: (#(#nones,)*),
                        key: std::marker::PhantomData
                    }
                }
            }
        }
    }).collect::<Vec<_>>();

    std::fs::write(dest_path, quote!(#(#out)*).to_string()).unwrap();



}