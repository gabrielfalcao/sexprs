use proc_macro2::{TokenStream, TokenTree};

use crate::Result;

pub trait ToTokensBuilder<T>: Sized + Clone {
    fn stream(&self) -> TokenStream;
    fn with_token(&self, token: TokenTree) -> Result<Self>;
    fn can_build(&self) -> bool;
    fn is_ready(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn build(&self) -> Result<T>;
    fn build_stream(&self) -> Result<TokenStream>;
}
