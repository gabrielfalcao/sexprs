use sexprs_util::with_caller;

use crate::{Error, ErrorType};

pub fn runtime_error(message: String, previous: Option<Error>) -> Error {
    with_caller!(Error::with_previous_error(message, ErrorType::RuntimeError, previous))
}
