#![allow(unused)]
extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Item;

use crate::{try_map_to_compile_error, EnumBuilder, Error};

#[derive(Debug, Clone)]
pub struct ErrorTypeNodeGen {
    attr: TokenStream,
    item: TokenStream,
}

impl ErrorTypeNodeGen {
    pub fn new(
        attr: TokenStream,
        item: TokenStream,
    ) -> Result<ErrorTypeNodeGen, TokenStream> {
        Ok(ErrorTypeNodeGen { attr, item })
    }

    pub fn attr(&self) -> TokenStream {
        self.attr.clone()
    }

    pub fn item(&self) -> TokenStream {
        self.item.clone()
    }

    pub fn code(&self) -> Result<TokenStream, TokenStream> {
        let item = match syn::parse2::<Item>(self.item()) {
            Ok(Item::Enum(item)) => {
                let builder = try_map_to_compile_error!(
                    EnumBuilder::from_token_stream(&item.to_token_stream())
                );
                let stream = try_map_to_compile_error!(builder.build());
                stream
            },
            Ok(item) => item.to_token_stream(),
            Err(e) =>
                try_map_to_compile_error!(Err::<TokenStream, Error>(e.into())),
        };

        Ok(item)
    }
}
