use std::fmt::{Debug, Display, Formatter};

#[rustfmt::skip]
use sexprs_parser as parser;
#[rustfmt::skip]
use sexprs_vm as vm;
#[rustfmt::skip]
use sexprs_formatter as formatter;
#[rustfmt::skip]
use sexprs_util::{impl_error, Caller, Traceback};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorType {
    IOError,
    ParserError,
    RuntimeError,
    FormatterError,
}
impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::IOError => "IOError",
                Self::ParserError => "ParserError",
                Self::RuntimeError => "RuntimeError",
                Self::FormatterError => "FormatterError",
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

impl From<parser::Error> for Error {
    fn from(e: parser::Error) -> Self {
        Error::new(e, ErrorType::ParserError)
    }
}
impl From<vm::Error> for Error {
    fn from(e: vm::Error) -> Self {
        Error::new(e, ErrorType::RuntimeError)
    }
}
impl From<formatter::Error> for Error {
    fn from(e: formatter::Error) -> Self {
        Error::new(e, ErrorType::FormatterError)
    }
}

#[derive(Debug, Clone)]
pub enum Exit {
    Success,
    Error(Error),
}
impl std::process::Termination for Exit {
    fn report(self) -> std::process::ExitCode {
        match &self {
            Exit::Success => std::process::ExitCode::from(0),
            Exit::Error(error) => {
                eprintln!("{}", error);
                std::process::ExitCode::from(1)
            },
        }
    }
}
impl<T> From<std::result::Result<T, Error>> for Exit {
    fn from(result: std::result::Result<T, Error>) -> Exit {
        match result {
            Ok(_) => Exit::Success,
            Err(e) => Exit::Error(e),
        }
    }
}
