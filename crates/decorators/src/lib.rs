#![allow(unused, non_snake_case)]
extern crate proc_macro;

use proc_macro2::{Literal, TokenStream, TokenTree};
use quote::quote;
use sexprs_formatter::{format_code, highlight_code, highlight_token_stream};
use sexprs_macros_core::{match_to_compile_error, ErrorTypeNodeGen};

#[proc_macro_attribute]
pub fn error_types(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr: TokenStream = attr.into();
    let item: TokenStream = item.into();

    let mut ast_node_gen = match_to_compile_error!(ErrorTypeNodeGen::new(attr.clone(), item.clone()));
    let code = match_to_compile_error!(ast_node_gen.code());
    // eprintln!("{}", highlight_code(&code).unwrap());
    code.into()
}
