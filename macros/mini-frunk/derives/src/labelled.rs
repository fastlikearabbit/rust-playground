#![forbid(unsafe_code)]
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::str::FromStr;
use syn::{parse_macro_input, Data, DeriveInput, FieldsNamed, Ident, Type};

pub fn impl_labelled(ast: &syn::DeriveInput) -> TokenStream {
    // TODO: your code goes here.
    todo!()
}

// TODO: your code goes here.
