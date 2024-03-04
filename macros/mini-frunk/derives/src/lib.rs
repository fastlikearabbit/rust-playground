#![forbid(unsafe_code)]
extern crate mini_frunk_core;

mod generic;
mod labelled;

use generic::impl_generic;
use labelled::impl_labelled;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(Generic)]
pub fn derive_generic(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input);
    impl_generic(ast)
}

#[proc_macro_derive(LabelledGeneric)]
pub fn derive_labelled(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_labelled(&ast)
}
