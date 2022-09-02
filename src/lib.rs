extern crate proc_macro;
mod generator;
mod reflection;

use proc_macro::TokenStream;
use syn;

#[proc_macro_derive(TableStorageEntity)]
pub fn postgres_select_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::generator::generate(&ast)
}
