extern crate proc_macro;
use sexprs_util::try_result;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Fields;

use crate::{EnumBuilder, Result};

impl EnumBuilder {
    pub fn build_enum_partial_ord_impl(&self) -> Result<TokenStream> {
        let name = self.name();
        let partial_ord_variants =
            try_result!(self.impl_partial_ord_variants());
        let stream = quote! {
            impl std::cmp::PartialOrd for #name {
                fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
                    #partial_ord_variants
                    None

                }
            }
        };
        Ok(stream)
    }

    pub fn impl_partial_ord_variants(&self) -> Result<TokenStream> {
        let name = self.name();
        let variants = self
            .variants()
            .into_iter()
            .map(|var| {
                let variant_ident = var.ident.clone();
                quote! {
                    if let #name::#variant_ident = self {
                        if let #name::#variant_ident = rhs {
                            return Some(std::cmp::Ordering::Equal)
                        }
                    }
                }
            })
            .collect::<Vec<TokenStream>>();
        let stream = quote! {
            #(

                #variants

            );*

        };
        Ok(stream)
    }
}
