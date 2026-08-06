use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};

#[proc_macro_attribute]
pub fn check_docs(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    // Your AST inspection/generation logic goes here

    let expanded = quote! {
        #input
    };

    TokenStream::from(expanded)
}
