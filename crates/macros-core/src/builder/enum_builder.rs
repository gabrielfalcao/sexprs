#![allow(unused)]
extern crate proc_macro;

use sexprs_util::{dbg, try_result};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{DeriveInput, Fields, Item, ItemEnum, Variant};

use crate::{Error, Result};


pub mod display;
pub mod debug;
pub mod variants;
pub mod partial_ord;
pub mod ord;
pub mod clone;
pub mod partial_eq;

#[derive(Clone)]
pub struct EnumBuilder {
    item: ItemEnum,
}
impl EnumBuilder {
    pub fn from_token_stream(stream: &TokenStream) -> Result<EnumBuilder> {
        if let Item::Enum(item) =
            try_result!(syn::parse2::<Item>(stream.clone()))
        {
            Ok(EnumBuilder { item: item.clone() })
        } else {
            Err(Error::compile_error("not an enum"))
        }
    }

    pub fn name(&self) -> Ident {
        self.item.ident.clone()
    }

    pub fn variants(&self) -> Punctuated<Variant, Comma> {
        let mut variants = Punctuated::<Variant, Comma>::new();
        for variant in self.item.variants.clone() {
            variants.push(variant.clone());
        }
        variants
    }

    pub fn build_enum(&self) -> Result<TokenStream> {
        let mut item = self.item.clone();
        item.variants = self.variants();
        Ok(item.to_token_stream())
    }

    pub fn ast(&self) -> Result<DeriveInput> {
        Ok(try_result!(syn::parse2::<DeriveInput>(
            try_result!(self.build_enum()).into()
        )))
    }

    pub fn build(&self) -> Result<TokenStream> {
        let name = self.name();

        let builder = self.clone();
        let enum_definition = try_result!(self.build_enum());

        let impl_variants = try_result!(builder.build_enum_variants_impl());
        let impl_debug = try_result!(builder.build_enum_debug_impl());
        let impl_display = try_result!(builder.build_enum_display_impl());
        let impl_ord = try_result!(builder.build_enum_ord_impl());
        let impl_partial_eq = try_result!(builder.build_enum_partial_eq_impl());

        let stream = quote!{
            #[derive(PartialEq, Clone, Eq, PartialOrd, Ord)]
            #enum_definition
            #impl_variants
            #impl_display
            #impl_debug
            #impl_partial_eq
        };
        Ok(TokenStream::from(stream))
    }
}
