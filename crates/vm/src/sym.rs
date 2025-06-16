use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt::{Debug, Formatter, Display};
use std::hash::{Hash, Hasher};

use sexprs_data_structures::{
    append, AsValue, Value,
};

use crate::Function;

#[derive(Clone)]
pub enum Sym<'c> {
    Value(Value<'c>),
    Function(Function<'c>),
}

impl<'c> Display for Sym<'c> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Sym::Value(value) => format!("{}", value),
                Sym::Function(function) => format!("{}", function),
            }
        )
    }
}
impl<'c> Debug for Sym<'c> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Sym::Value(value) => format!("{:#?}", value),
                Sym::Function(function) => format!("{:#?}", function),
            }
        )
    }
}
impl<'c> Sym<'c> {
    pub fn as_value(&self) -> Value<'c> {
        match self {
            Sym::Value(value) => value.clone(),
            Sym::Function(Function::Builtin { name, function }) => Value::symbol(name),
            Sym::Function(Function::Defun { name, args, body }) => Value::list([
                Value::from(name),
                args.clone(),
                append(body.clone()),
            ]),
        }
    }
}
impl<'c> Hash for Sym<'c> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_value().hash(state)
    }
}
impl<'c> PartialEq for Sym<'c> {
    fn eq(&self, other: &Sym<'c>) -> bool {
        self.as_value().eq(&other.as_value())
    }
}
impl<'c> PartialOrd for Sym<'c> {
    fn partial_cmp(&self, other: &Sym<'c>) -> Option<Ordering> {
        self.as_value().partial_cmp(&other.as_value())
    }
}
