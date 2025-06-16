use std::fmt::{Debug, Display, Formatter};

pub use sexprs_formatter::Error as FormatterError;
pub use sexprs_util::{color, with_caller, Caller, Traceback};

use crate::Span;

#[derive(Clone, PartialEq, Eq)]
pub struct Error<'a> {
    message: String,
    info: Option<Span<'a>>,
    callers: Vec<Caller>,
}
impl<'a> Error<'a> {
    pub fn new<T: Display>(message: T, info: Option<Span<'a>>) -> Self {
        let message = message.to_string();
        Error {
            message: message,
            info,
            callers: Vec::new(),
        }
    }
}
impl std::error::Error for Error<'_> {}
impl<'a> Traceback for Error<'a> {
    fn message(&self) -> String {
        self.message.to_string()
    }

    fn with(&self, caller: Caller) -> Self {
        let mut error = self.clone();
        error.callers.insert(0, caller);
        error
    }

    fn callers(&self) -> Vec<Caller> {
        self.callers.to_vec()
    }
}
impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            [
                color::reset(""),
                if let Some(info) = &self.info {
                    color::bg(info.highlight_input(4), 198)
                } else {
                    String::new()
                },
                color::fg(format!("\n\rreason: {}", self.highlight_message()), 242),
            ]
            .join("")
        )
    }
}
impl<'a> Debug for Error<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            [
                color::reset(""),
                color::fg(format!("in source:\n{}", self.to_string()), 202),
                if self.callers.is_empty() {
                    String::new()
                } else {
                    [
                        color::reset("\n\n"),
                        color::fg(format!("Stacktrace:\n{}\n", self.callers_to_string(4)), 220),
                    ]
                    .join("")
                }
            ]
            .join("")
        )
    }
}
impl<'a> From<FormatterError> for Error<'a> {
    fn from(e: FormatterError) -> Error<'a> {
        Error::new(e.to_string(), None)
    }
}
pub type Result<'a, T> = std::result::Result<T, Error<'a>>;
