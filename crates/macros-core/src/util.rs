use proc_macro2::{Ident, TokenStream};

pub fn ident_to_string(ident: &Ident) -> String {
    format!("{}", ident)
}

pub fn compile_error<E: std::fmt::Display>(error: E) -> TokenStream {
    syn::Error::new(proc_macro2::Span::call_site(), error.to_string()).to_compile_error()
}
