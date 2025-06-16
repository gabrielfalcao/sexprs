extern crate proc_macro;
use sexprs_util::try_result;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::Fields;

use crate::{ident_to_string, EnumBuilder, Result};

impl EnumBuilder {
    pub fn build_enum_variants_impl(&self) -> Result<TokenStream> {
        let name = self.name();
        let count = Literal::usize_unsuffixed(self.variants().len());
        let str_variants = try_result!(self.impl_variants_variants());
        let stream = quote! {
            impl #name {
                fn variants() -> [&'static str; #count] {
                    [#str_variants]
                }
            }
        };
        Ok(stream)
    }

    pub fn impl_variants_variants(&self) -> Result<TokenStream> {
        let name = self.item.ident.clone();
        let enum_name = format!("{}", name);
        let variants = self
            .variants()
            .into_iter()
            .map(|var| {
                let variant_ident = var.ident.clone();
                let variant_name = ident_to_string(&var.ident);
                quote! {
                    #variant_name
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
