extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::quote;
use sexprs_util::try_result;
use syn::Fields;

use crate::{EnumBuilder, Result};

impl EnumBuilder {
    pub fn build_enum_partial_eq_impl(&self) -> Result<TokenStream> {
        let name = self.name();
        let partial_eq_string_variants = try_result!(self.impl_partial_eq_string_variants());
        let stream = quote! {
            // impl std::cmp::PartialEq for #name {
            //     fn eq(&self, rhs: &#name) -> bool {
            //         #partial_eq_variants
            //     }
            // }
            // impl std::cmp::PartialEq<&#name> for #name {
            //     fn eq(&self, rhs: &&#name) -> bool {
            //         let rhs = *rhs;
            //         #partial_eq_variants
            //     }
            // }
            // impl std::cmp::PartialEq<&#name> for #name {
            //     fn eq(&self, rhs: &#name) -> bool {
            //         self.eq(*rhs)
            //     }
            // }
            // impl std::cmp::PartialEq for &#name {
            //     fn eq(&self, rhs: &#name) -> bool {
            //         (*self).eq(rhs)
            //     }
            // }
            impl std::cmp::PartialEq<str> for #name {
                fn eq(&self, rhs: &str) -> bool {
                    #partial_eq_string_variants
                }
            }
            impl std::cmp::PartialEq<String> for #name {
                fn eq(&self, rhs: &String) -> bool {
                    #partial_eq_string_variants
                }
            }
        };
        Ok(stream)
    }

    pub fn impl_partial_eq_variants(&self) -> Result<TokenStream> {
        let name = self.name();
        let variants = self.variants()
            .into_iter()
            .map(|var| {
                let variant_ident = var.ident.clone();

                quote! {
                    if self == #name::#variant_ident && rhs == #name::#variant_ident {
                        return true
                    }
                }
            })
            .collect::<Vec<TokenStream>>();
        let stream = quote! {
            #(

                #variants

            );*
            false
        };

        Ok(stream)
    }
    pub fn impl_partial_eq_string_variants(&self) -> Result<TokenStream> {
        let name = self.name();
        let variants = self.variants()
            .into_iter()
            .map(|var| {
                let variant_ident = var.ident.clone();

                quote! {
                    if &self.to_string() == rhs {
                        return true
                    }
                }
            })
            .collect::<Vec<TokenStream>>();
        let stream = quote! {
            #(

                #variants

            );*
            false
        };

        Ok(stream)
    }
}
