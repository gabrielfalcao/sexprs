#![allow(unused, non_snake_case)]
use k9::assert_equal;
use sexprs_decorators::error_types;

#[error_types]
pub enum ErrorType {
    CompileError,
    RuntimeError,
}

#[test]
fn test_variants() {
    assert_equal!(ErrorType::variants(), ["CompileError", "RuntimeError"]);
}
#[test]
fn test_display() {
    assert_equal!(format!("{}", ErrorType::CompileError), "CompileError");
    assert_equal!(format!("{}", ErrorType::RuntimeError), "RuntimeError");
}
#[test]
fn test_debug() {
    assert_equal!(format!("{:#?}", ErrorType::CompileError), "ErrorType::CompileError");
    assert_equal!(format!("{:#?}", ErrorType::RuntimeError), "ErrorType::RuntimeError");
}
#[test]
fn test_eq() {
    assert_equal!(ErrorType::CompileError, ErrorType::CompileError);
    assert_ne!(ErrorType::CompileError, ErrorType::RuntimeError);
}
// #[test]
// fn test_clone() {
//     assert_equal!(ErrorType::CompileError, ErrorType::CompileError.clone());
// }
