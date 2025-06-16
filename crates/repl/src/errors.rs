use sexprs_util::{impl_error, Traceback};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum ErrorType {
    IOError,
    SyntectError,
    FormatError,
    RuntimeError,
    FormatterError,
    ReplError,
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
                Self::RuntimeError => "RuntimeError",
                Self::FormatterError => "FormatterError",
                Self::ReplError => "ReplError",
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
// impl From<rustyline::Error> for Error {
//     fn from(e: rustyline::Error) -> Self {
//         Error::new(e, ErrorType::ReplError)
//     }
// }
impl From<rustyline::error::ReadlineError> for Error {
    fn from(e: rustyline::error::ReadlineError) -> Self {
        Error::new(e, ErrorType::ReplError)
    }
}

impl From<sexprs_vm::Error> for Error {
    fn from(e: sexprs_vm::Error) -> Self {
        Error::new(e, ErrorType::RuntimeError)
    }
}

impl From<sexprs_formatter::Error> for Error {
    fn from(e: sexprs_formatter::Error) -> Self {
        Error::new(e, ErrorType::FormatterError)
    }
}
