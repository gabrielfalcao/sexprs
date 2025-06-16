#![feature(non_null_from_ref)]
extern crate proc_macro;

pub mod attr_macro;
pub use attr_macro::ErrorTypeNodeGen;
pub mod test;
pub mod util;
pub use util::{compile_error, ident_to_string};
pub mod builder;
pub use builder::{EnumBuilder, ToTokensBuilder};
pub mod errors;
pub use errors::{Error, ErrorType, Result};
pub mod ordered_string_set;
pub use ordered_string_set::OrderedStringSet;

#[macro_export]
macro_rules! try_map_to_compile_error {
    ($result:expr) => {
        $crate::map_to_compile_error!($result)?
    };
}

#[macro_export]
macro_rules! map_to_compile_error {
    ($result:expr) => {
        $result.map_err(|error| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("{}\n@{}:{}", error.to_string(), file!(), line!(),),
            )
            .to_compile_error()
        })
    };
}

#[macro_export]
macro_rules! match_to_compile_error {
    ($result:expr) => {
        match $crate::map_to_compile_error!($result) {
            Ok(result) => result,
            Err(error) => return error.into(),
        }
    };
}
