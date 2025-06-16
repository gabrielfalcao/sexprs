extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::quote;
use sexprs_util::try_result;
use syn::Fields;

use crate::{EnumBuilder, Result};

impl EnumBuilder {
    pub fn build_enum_clone_impl(&self) -> Result<TokenStream> {
        let name = self.name();
        let clone_variants = try_result!(self.impl_clone_variants());
        let stream = quote! {
            impl std::clone::Clone for #name {
                fn clone(&self) -> Self {
                    match self {
                        #clone_variants
                    }
                }
            }
            impl std::marker::Copy for #name {}
        };
        Ok(stream)
    }

    pub fn impl_clone_variants(&self) -> Result<TokenStream> {
        let name = self.name();
        let variants = self
            .variants()
            .into_iter()
            .map(|var| {
                let variant_ident = var.ident.clone();
                quote! {
                    #name::#variant_ident => #name::#variant_ident
                }
            })
            .collect::<Vec<TokenStream>>();
        let stream = quote! {
            #(
                #variants
            ),*
        };

        Ok(stream)
    }
}
