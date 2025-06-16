use std::cmp::{Eq, PartialEq};

use proc_macro2::{TokenStream, TokenTree};
use sexprs_formatter::{format_code_naive, Error as FormatterError};
use sexprs_util::{impl_error, Traceback};

#[derive(Clone)]
pub enum ErrorType {
    BuilderError(TokenStream),
    FormatterError(FormatterError),
    CompileError(TokenStream),
    TokenStreamError(TokenStream),
    TokenTreeError(TokenTree),
    SynError(syn::Error),
}
impl PartialEq for ErrorType {
    fn eq(&self, other: &ErrorType) -> bool {
        format!("{:#?}", self) == format!("{:#?}", other)
    }
}
impl Eq for ErrorType {}
impl ErrorType {
    pub fn variant(&self) -> String {
        match self {
            Self::BuilderError(_) => "BuilderError",
            Self::CompileError(_) => "CompileError",
            Self::FormatterError(_) => "FormatterError",
            Self::TokenStreamError(_) => "TokenStreamError",
            Self::TokenTreeError(_) => "TokenTreeError",
            Self::SynError(_) => "SynError",
        }
        .to_string()
    }

    pub fn surround<T: std::fmt::Display>(&self, error: T) -> String {
        format!("{}: {}", self.variant(), error)
    }
}
impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::CompileError(ts) => format!("{}", ts),
                Self::BuilderError(ts) => self.surround(ts),
                Self::FormatterError(e) => e.to_string(),
                Self::TokenStreamError(ts) => self.surround(ts),
                Self::TokenTreeError(tt) => self.surround(tt),
                Self::SynError(e) => e.to_string(),
            }
        )
    }
}
impl std::fmt::Debug for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::BuilderError(ts) => format_code_naive(format!("{:#?}", ts)),
                Self::CompileError(ts) => format_code_naive(format!("{:#?}", ts)),
                Self::TokenStreamError(ts) => format_code_naive(format!("{:#?}", ts)),
                Self::TokenTreeError(tt) => format_code_naive(format!("{:#?}", tt)),
                Self::SynError(e) => format!("{:#?}", e),
                Self::FormatterError(e) => format!("{:#?}", e),
            }
        )
    }
}

impl_error!(Error, ErrorType);
impl Error {
    pub fn from_builder<T: std::fmt::Display>(error: T, stream: &TokenStream) -> Error {
        Error::new(format_code_naive(error), ErrorType::BuilderError(stream.clone()))
    }

    pub fn compile_error<T: std::fmt::Display>(error: T) -> Error {
        Error::new(
            format_code_naive(error.to_string()),
            ErrorType::CompileError(crate::util::compile_error(error.to_string())),
        )
    }
}
impl From<syn::Error> for Error {
    fn from(e: syn::Error) -> Self {
        Error::new(e.clone(), ErrorType::CompileError(e.to_compile_error()))
    }
}
impl From<TokenStream> for Error {
    fn from(ts: TokenStream) -> Self {
        Error::new(ts.clone(), ErrorType::TokenStreamError(ts.clone()))
    }
}

impl From<TokenTree> for Error {
    fn from(ts: TokenTree) -> Self {
        Error::new(ts.clone(), ErrorType::TokenTreeError(ts.clone()))
    }
}

impl From<FormatterError> for Error {
    fn from(ts: FormatterError) -> Self {
        Error::new(ts.clone(), ErrorType::FormatterError(ts.clone()))
    }
}
