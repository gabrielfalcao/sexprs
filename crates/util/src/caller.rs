use std::fmt::{Debug, Display, Formatter};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Caller(pub String, pub String, pub u32);
impl Caller {
    pub fn function_name(&self) -> String {
        self.0.to_string()
    }

    pub fn file(&self) -> String {
        self.1.to_string()
    }

    pub fn line(&self) -> u32 {
        self.2.clone()
    }
}
impl Display for Caller {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "    \x1b[1;38;5;79m{}\x1b[1;38;5;235m @ \x1b[1;38;5;159m{}:{}\x1b[0m",
            self.function_name(),
            self.file(),
            self.line()
        )
    }
}
