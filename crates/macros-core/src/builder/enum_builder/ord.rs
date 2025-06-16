extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::quote;
use sexprs_util::try_result;
use syn::Fields;

use crate::{EnumBuilder, Result};

impl EnumBuilder {
    pub fn build_enum_ord_impl(&self) -> Result<TokenStream> {
        let name = self.name();
        let ord_variants = try_result!(self.impl_ord_variants());
        let stream = quote! {
            impl std::cmp::Ord for #name {
                fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
                    #ord_variants
                    return std::cmp::Ordering::Less
                }
            }
        };
        Ok(stream)
    }

    pub fn impl_ord_variants(&self) -> Result<TokenStream> {
        let name = self.name();
        let variants = self
            .variants()
            .into_iter()
            .map(|var| {
                let variant_ident = var.ident.clone();

                quote! {
                    if let #name::#variant_ident = self {
                        if let #name::#variant_ident = rhs {
                            return std::cmp::Ordering::Equal
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
