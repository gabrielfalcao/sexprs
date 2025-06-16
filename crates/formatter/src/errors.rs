use sexprs_util::{impl_error, Traceback};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum ErrorType {
    IOError,
    SyntectError,
    FormatError,
    SynError,
}
impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::IOError => "IOError",
                Self::SyntectError => "SyntectError",
                Self::FormatError => "FormatError",
                Self::SynError => "SynError",
            }
        )
    }
}
impl_error!(Error, ErrorType);
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::new(e, ErrorType::IOError)
    }
}
impl From<iocore::Error> for Error {
    fn from(e: iocore::Error) -> Self {
        Error::new(e, ErrorType::IOError)
    }
}

impl From<syntect::LoadingError> for Error {
    fn from(e: syntect::LoadingError) -> Self {
        Error::new(e, ErrorType::SyntectError)
    }
}
impl From<syntect::Error> for Error {
    fn from(e: syntect::Error) -> Self {
        Error::new(e, ErrorType::SyntectError)
    }
}
impl From<syn::Error> for Error {
    fn from(e: syn::Error) -> Self {
        Error::new(e, ErrorType::SynError)
    }
}
