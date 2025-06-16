extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::quote;
use sexprs_util::try_result;
use syn::Fields;

use crate::{ident_to_string, EnumBuilder, Result};

impl EnumBuilder {
    pub fn build_enum_debug_impl(&self) -> Result<TokenStream> {
        let name = self.name();
        let str_variants = try_result!(self.impl_debug_variants());
        let stream = quote! {
            impl std::fmt::Debug for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(
                        f,
                        "{}",
                        match self {
                            #str_variants
                        }
                    )
                }
            }
        };
        Ok(stream)
    }

    pub fn impl_debug_variants(&self) -> Result<TokenStream> {
        let name = self.item.ident.clone();
        let enum_name = format!("{}", name);
        let variants = self
            .variants()
            .into_iter()
            .map(|var| {
                let variant_ident = var.ident.clone();
                let variant_name = [enum_name.clone(), ident_to_string(&var.ident)].join("::");
                quote! {
                    #name::#variant_ident => #variant_name
                }
            })
            .collect::<Vec<TokenStream>>();

        let stream = quote! {
            #(
              #variants),*
        };

        Ok(stream)
    }
}
