#![forbid(unsafe_code)]
use core::panic;

use proc_macro::TokenStream;
use quote::quote;

pub fn impl_labelled(ast: syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let input_fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed {ref named, ..}),
        ..
    }) = ast.data {
        named
    } else {
        panic!("Not a struct with named fields.")
    };

    let struct_ = match &ast.data {
        syn::Data::Struct(struct_) => struct_,
        _ => panic!("Only structs are accepted.")
    };  

    let field_names_iter = match &struct_.fields {
        syn::Fields::Named(fields) => {
            let names = fields.named.iter().map(|f| {
                let name = f.ident.as_ref().unwrap();
                quote! { #name }
            });
            quote! { #(#names),* }
        },

        _ => panic!("balblablbla")
    };

    let custom_field_items = input_fields.iter().map(|f| {
            let name_as_iter = f.ident.clone().into_iter();
            let enum_based_name = quote! { #(#name_as_iter),* };

            quote! { Field<#enum_based_name, #f.ty.clone()> }
        }
    );

    let field_names = input_fields.iter().map(|f| quote! { #f.ident.clone() });
    let field_names_cpy = field_names.clone();

    let generics = ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let impl_block = quote! {
            impl #impl_generics ::mini_frunk_core::labelled::LabelledGeneric for #name #ty_generics #where_clause {
                type Repr = HList![ #(#custom_field_items),* ];

                fn into(self) -> Self::Repr {
                    // hlist![
                    //     quote! { #(field!(#field_names, self.#field_names)),* }
                    // ]
                    todo!()
                }

                fn from(repr: Self::Repr) -> Self {
                    // let hlist_pat![ #field_names_iter ] = repr;
                    // let comma_separated_list = #(#field_names_cpy: #field_names_cpy.value),*;
                    // Self {
                    //     quote! { comma_separated_list }
                    // }
                    todo!()
                }
        }
    };
    impl_block.into()
}
