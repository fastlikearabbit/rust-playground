#![forbid(unsafe_code)]
use proc_macro::TokenStream;
use quote::quote;

pub fn impl_generic(ast: syn::DeriveInput) -> TokenStream {
    let struct_ = match ast.data {
                        syn::Data::Struct(struct_) => struct_,
                        _ => panic!("Only structs are accepted.")
    };
    let name = ast.ident;
    let generics = ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let (field_names, field_types) = match struct_.fields {
        syn::Fields::Named(fields) => {
            let names = fields.named.iter().map(|f| {
                let name = f.ident.as_ref().unwrap();
                quote! { #name }
            });
            let types = fields.named.iter().map(|f| {
                let ty = &f.ty;
                quote! { #ty }
            });
            (quote! { #(#names),* }, quote! { #(#types),* })
        },
        _ => panic!("Only named fields accepted.")
    };

    let impl_block = quote! {
        impl #impl_generics ::mini_frunk_core::generic::Generic for #name #ty_generics #where_clause {
            type Repr = HList!( #field_types );

            fn into(self) -> Self::Repr {
                let #name { #field_names } = self;
                hlist! [ #field_names ]
            }

            fn from(repr: Self::Repr) -> Self {
                let hlist_pat! [ #field_names ] = repr;
                #name { #field_names }
            }
        }
    };

    impl_block.into()
}

// TODO: your code goes here.
