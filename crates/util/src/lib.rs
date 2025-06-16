#![allow(unused, non_snake_case)]
pub mod caller;
pub mod color;
pub mod macros;
pub use caller::Caller;

pub trait Traceback: std::error::Error {
    fn message(&self) -> String;
    fn with(&self, caller: crate::Caller) -> Self;
    fn callers(&self) -> Vec<crate::Caller>;

    fn callers_to_string(&self, indent: usize) -> String {
        let indentation = " ".repeat(indent).to_string();
        self.callers()
            .iter()
            .enumerate()
            .map(|(index, caller)| format!("{}{}", indentation.repeat(index), caller))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn highlight_message(&self) -> String {
        format!("{}", self.message())
    }
    fn previous_as_debug(&self) -> String {
        String::new()
    }
    fn previous_as_string(&self) -> String {
        String::new()
    }
}
