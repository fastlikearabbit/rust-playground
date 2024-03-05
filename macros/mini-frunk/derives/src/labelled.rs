#![forbid(unsafe_code)]
use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;


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

    let custom_field_items = input_fields
        .iter()
        .map(|f| {
            let as_string = f.ident.clone().unwrap().to_string();
            let letters = as_string.chars().into_iter().map(|c| {
                if c == '_' { 
                    syn::Ident::new("__", f.span())
                } else {
                    syn::Ident::new(&c.to_string(), f.span()) 
                }
            })
            .map(|ident| quote! { mini_frunk_core::field::symbols::#ident });

            let as_enum = quote! { #(#letters),* };    
            let ty = f.ty.clone();

            quote! { mini_frunk_core::field::Field<(#as_enum), #ty> }
    });

    let field_names_as_enums = input_fields
        .iter()
        .map(|f| {
            let as_string = f.ident.clone().unwrap().to_string();
            let letters = as_string.chars().into_iter().map(|c| {
                if c == '_' { 
                    syn::Ident::new("__", f.span())
                } else {
                    syn::Ident::new(&c.to_string(), f.span()) 
                }
            })
            .map(|ident| quote! { mini_frunk_core::field::symbols::#ident });

            let as_enum = quote! { #(#letters),* };   
            quote! { (#as_enum) }
    });
    let field_names = input_fields.iter().map(|f| {
        let ident = f.ident.clone();
        quote! { #ident }
    });
    let field_names_cpy = field_names.clone();
    let self_field_values = input_fields.iter().map(|f| {
        let field_name = f.ident.clone().unwrap();
        quote! { self.#field_name }
    });

    // println!("{:#?}", quote! {#(#field_names),*});

    let generics = ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let impl_block = quote! {
            impl #impl_generics ::mini_frunk_core::labelled::LabelledGeneric for #name #ty_generics #where_clause {
                type Repr = mini_frunk_core::HList![ #(#custom_field_items),* ];

                fn into(self) -> Self::Repr {
                    mini_frunk_core::hlist![
                        #(mini_frunk_core::field!(#field_names_as_enums, #self_field_values)),*
                    ]
                }

                fn from(repr: Self::Repr) -> Self {
                    let mini_frunk_core::hlist_pat![ #(#field_names),* ] = repr;
                    Self {
                        #(#field_names_cpy: #field_names_cpy.value),*
                    }
                }
        }
    };
    impl_block.into()
}
