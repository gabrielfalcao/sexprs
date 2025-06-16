#[derive(PartialEq, Clone, Eq, PartialOrd, Ord)]
pub enum Value {
    CompileError,
    RuntimeError,
}
impl Value {
    fn variants() -> [&'static str; 2] {
        ["CompileError", "RuntimeError"]
    }
}
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::CompileError => "CompileError",
                Value::RuntimeError => "RuntimeError",
            }
        )
    }
}
impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::CompileError => "Value::CompileError",
                Value::RuntimeError => "Value::RuntimeError",
            }
        )
    }
}
impl std::cmp::PartialEq<str> for Value {
    fn eq(&self, rhs: &str) -> bool {
        if &self.to_string() == rhs {
            return true;
        };
        if &self.to_string() == rhs {
            return true;
        }
        false
    }
}
impl std::cmp::PartialEq<String> for Value {
    fn eq(&self, rhs: &String) -> bool {
        if &self.to_string() == rhs {
            return true;
        };
        if &self.to_string() == rhs {
            return true;
        }
        false
    }
}
