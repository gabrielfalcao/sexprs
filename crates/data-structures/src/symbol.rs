#![allow(unused)]
use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};

use unique_pointer::UniquePointer;

use crate::{AsValue, Quotable, Value};

pub trait AsSymbol<'c> {
    fn as_symbol(&self) -> Symbol<'c>;
    fn is_symbol(&self) -> bool {
        true
    }
}

#[derive(Clone, PartialOrd, Ord, Default, Eq, Hash)]
pub struct Symbol<'c> {
    sym: &'c str,
    quoted: bool,
}
impl<'c> Symbol<'c> {
    pub fn new<T: ToString>(sym: T) -> Symbol<'c> {
        Symbol::quoted(sym, false)
    }

    pub fn quoted<T: ToString>(sym: T, quoted: bool) -> Symbol<'c> {
        Symbol {
            sym: sym.to_string().leak(),
            quoted,
        }
    }

    pub fn symbol(&self) -> &'c str {
        self.sym
    }

    pub fn quote(&self) -> Symbol<'c> {
        Symbol::quoted(self.symbol(), true)
    }

    pub fn unquote(&self) -> Symbol<'c> {
        Symbol::quoted(self.symbol(), false)
    }

    pub fn is_quoted(&self) -> bool {
        self.quoted
    }
}

impl Display for Symbol<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.sym)
    }
}
impl Debug for Symbol<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.sym)
    }
}

impl<'c> From<&'c str> for Symbol<'c> {
    fn from(symbol: &'c str) -> Symbol<'c> {
        Symbol::new(symbol)
    }
}

impl<'c> From<String> for Symbol<'c> {
    fn from(symbol: String) -> Symbol<'c> {
        Symbol::new(&symbol)
    }
}

impl<'c> AsSymbol<'c> for Symbol<'c> {
    fn as_symbol(&self) -> Symbol<'c> {
        self.clone()
    }
}
impl<'c> AsSymbol<'c> for &Symbol<'c> {
    fn as_symbol(&self) -> Symbol<'c> {
        UniquePointer::read_only(*self).read()
    }
}

impl<'c> AsSymbol<'c> for String {
    fn as_symbol(&self) -> Symbol<'c> {
        Symbol::new(self)
    }
}

impl<'c> AsSymbol<'c> for &str {
    fn as_symbol(&self) -> Symbol<'c> {
        Symbol::new(*self)
    }
}

impl<'c> AsSymbol<'c> for str {
    fn as_symbol(&self) -> Symbol<'c> {
        Symbol::new(self)
    }
}
impl<'c> AsSymbol<'c> for Cow<'c, str> {
    fn as_symbol(&self) -> Symbol<'c> {
        Symbol::new(&self)
    }
}
impl<'c> AsSymbol<'c> for &Cow<'c, str> {
    fn as_symbol(&self) -> Symbol<'c> {
        Symbol::new(*self)
    }
}

impl<'c> Quotable for Symbol<'c> {
    fn is_quoted(&self) -> bool {
        self.quoted
    }

    fn set_quoted(&mut self, quoted: bool) {
        self.quoted = quoted;
    }
}
impl<'c> AsValue<'c> for Symbol<'c> {
    fn as_value(&self) -> Value<'c> {
        Value::Symbol(self.clone())
    }
}

impl<'c> std::cmp::PartialEq for Symbol<'c> {
    fn eq(&self, rhs: &Symbol<'c>) -> bool {
        self.symbol() == rhs.symbol()
    }
}

// // impl<'c> AsRef<Symbol<'c>> for Symbol<'c> {
// //     fn as_ref(&self) -> &Symbol<'c> {
// //         self
// //     }
// // }
// //
// // impl<'c, T: AsRef<Symbol<'c>>> AsSymbol<'c> for T {
// //     fn as_symbol(&self) -> Symbol<'c> {
// //         self.as_ref().clone()
// //     }
// // }
