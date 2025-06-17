#![allow(unused)]
use std::borrow::Cow;
use std::convert::{AsMut, AsRef};
use std::fmt::{Debug, Display, Formatter};
use std::iter::{Extend, FromIterator, IntoIterator};

use unique_pointer::UniquePointer;

pub mod integer;
mod macros;
pub use integer::{AsInteger, Integer};
pub mod float;
pub use float::{AsFloat, Float};
pub mod unsigned_integer;
use sexprs_util::{dbg, try_result};
pub use unsigned_integer::{AsUnsignedInteger, UnsignedInteger};

use crate::{AsCell, AsNumber, AsSymbol, Cell, ListIterator, Quotable, Symbol};

pub trait ValueListIterator<'c>: IntoIterator<Item = Value<'c>> + Quotable {}
// impl <'c, T: IntoIterator<Item = Value<'c>> + Quotable> ValueListIterator<'c> for T {}

pub trait AsValue<'c>: Quotable {
    fn as_value(&self) -> Value<'c>;
}

#[derive(Clone, Ord, Default, Eq, Hash, PartialOrd, PartialEq)]
pub enum Value<'c> {
    #[default]
    Nil,
    T,
    String(&'c str),
    Symbol(Symbol<'c>),
    QuotedSymbol(Symbol<'c>),
    Byte(u8),
    UnsignedInteger(UnsignedInteger),
    Integer(Integer),
    Float(Float),
    List(Cell<'c>),
    QuotedList(Cell<'c>),
    EmptyList,
    EmptyQuotedList,
}
impl<'c> Value<'c> {
    pub fn nil() -> Value<'c> {
        Value::Nil
    }

    pub fn t() -> Value<'c> {
        Value::T
    }

    pub fn symbol<T: AsSymbol<'c>>(sym: T) -> Value<'c> {
        Value::Symbol(sym.as_symbol().unquote())
    }

    pub fn quoted_symbol<T: AsSymbol<'c>>(sym: T) -> Value<'c> {
        Value::QuotedSymbol(sym.as_symbol().quote())
    }

    pub fn string<T: ToString>(value: T) -> Value<'c> {
        Value::String(value.to_string().leak())
    }

    pub fn byte<T: AsNumber<u8>>(byte: T) -> Value<'c> {
        Value::Byte(byte.as_number())
    }

    pub fn unsigned_integer<T: AsUnsignedInteger>(value: T) -> Value<'c> {
        Value::UnsignedInteger(value.as_unsigned_integer())
    }

    pub fn integer<T: AsInteger>(value: T) -> Value<'c> {
        Value::Integer(value.as_integer())
    }

    pub fn float<T: AsFloat>(value: T) -> Value<'c> {
        Value::Float(value.as_float())
    }

    pub fn list<T: AsCell<'c>>(item: T) -> Value<'c> {
        if item.is_quoted() {
            Value::QuotedList(item.as_cell().quote())
        } else {
            Value::List(item.as_cell().unquote())
        }
    }

    pub fn quoted_list<T: AsCell<'c>>(item: T) -> Value<'c> {
        Value::QuotedList(item.as_cell().quote())
    }

    pub fn is_nil(&self) -> bool {
        if *self == Value::Nil {
            true
        } else {
            false
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Value::List(h) => h.is_nil(),
            Value::QuotedList(h) => h.is_nil(),
            Value::EmptyList => true,
            Value::EmptyQuotedList => true,
            Value::Nil => true,
            _ => false,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Value::List(h) => h.len(),
            Value::QuotedList(h) => h.len(),
            Value::EmptyList => 0,
            Value::EmptyQuotedList => 0,
            Value::Nil => 0,
            _ => 0,
        }
    }

    pub fn empty_list() -> Value<'c> {
        Value::EmptyList
    }

    pub fn empty_quoted_list() -> Value<'c> {
        Value::EmptyQuotedList
    }

    pub fn quote(&self) -> Value<'c> {
        let value = match self {
            Value::Symbol(h) => Value::QuotedSymbol(h.clone().unquote()),
            Value::List(h) => Value::QuotedList(h.clone().unquote()),
            Value::QuotedSymbol(h) => Value::QuotedSymbol(h.clone().quote()),
            Value::QuotedList(h) => Value::QuotedList(h.clone().quote()),
            _ => self.clone(),
        };
        value.clone()
    }

    pub fn values(&self) -> Vec<Value<'c>> {
        match self {
            Value::List(cell) | Value::QuotedList(cell) => cell.values(),
            _ => Vec::new(),
        }
    }

    pub fn head(&self) -> Value<'c> {
        match self {
            Value::List(cell) | Value::QuotedList(cell) => cell.head().unwrap_or_default(),
            _ => Value::nil(),
        }
    }

    pub fn tail(&self) -> Cell<'c> {
        match self {
            Value::List(cell) | Value::QuotedList(cell) => {
                cell.tail().map(Clone::clone).unwrap_or_default()
            }
            _ => Cell::nil(),
        }
    }

    pub fn wrap_in_list(&self) -> Value<'c> {
        if self.is_quoted() {
            Value::QuotedList(Cell::new(self.clone()))
        } else {
            Value::List(Cell::new(self.clone()))
        }
    }

    pub fn unwrap_list(&self) -> Value<'c> {
        match self {
            Value::List(cell) | Value::QuotedList(cell) => {
                if cell.tail.is_null() {
                    let value = cell.head().unwrap_or_default();
                    value.clone()
                } else {
                    self.clone()
                }
            }
            _ => self.clone(),
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Value::Integer(_) => true,
            _ => false,
        }
    }

    pub fn is_unsigned_integer(&self) -> bool {
        match self {
            Value::UnsignedInteger(_) => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Value::Float(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Value::String(_) => true,
            _ => false,
        }
    }

    pub fn is_symbol(&self) -> bool {
        match self {
            Value::Symbol(_) => true,
            Value::QuotedList(_) => true,
            _ => false,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            Value::List(_) => true,
            Value::QuotedList(_) => true,
            _ => false,
        }
    }
}

impl<'c> AsValue<'c> for Value<'c> {
    fn as_value(&self) -> Value<'c> {
        self.clone()
    }
}
impl<'c> AsValue<'c> for &Value<'c> {
    fn as_value(&self) -> Value<'c> {
        UniquePointer::read_only(*self).read()
    }
}

impl<'c> Drop for Value<'c> {
    fn drop(&mut self) {}
}

impl Debug for Value<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl Display for Value<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::T => "t".to_string(),
                Value::Nil => "nil".to_string(),
                Value::Byte(h) => format!("0x{:02x}", h),
                Value::Float(h) => format!("{}", h),
                Value::Integer(h) => format!("{}", h),
                Value::String(h) => format!("{:#?}", h),
                Value::Symbol(h) => format!("{}", h),
                Value::QuotedSymbol(h) => format!("'{}", h),
                Value::UnsignedInteger(h) => format!("{}", h),
                Value::List(h) => {
                    if h.is_nil() {
                        format!("()")
                    } else {
                        format!("({})", h)
                    }
                }
                Value::QuotedList(h) => {
                    if h.is_nil() {
                        format!("'()")
                    } else {
                        format!("'({})", h)
                    }
                }
                Value::EmptyList => format!("()"),
                Value::EmptyQuotedList => format!("'()"),
            }
        )
    }
}
impl<'c> From<()> for Value<'c> {
    fn from(_: ()) -> Value<'c> {
        Value::Nil
    }
}
impl<'c> Into<bool> for &Value<'c> {
    fn into(self) -> bool {
        !self.is_empty()
    }
}
impl<'c> Into<bool> for Value<'c> {
    fn into(self) -> bool {
        !self.is_empty()
    }
}
impl<'c> From<bool> for Value<'c> {
    fn from(value: bool) -> Value<'c> {
        if value {
            Value::T
        } else {
            Value::Nil
        }
    }
}
impl<'c> From<u8> for Value<'c> {
    fn from(value: u8) -> Value<'c> {
        Value::Byte(value)
    }
}
impl<'c> From<Symbol<'c>> for Value<'c> {
    fn from(value: Symbol<'c>) -> Value<'c> {
        if value.is_quoted() {
            Value::quoted_symbol(value.quote())
        } else {
            Value::symbol(value.unquote())
        }
    }
}
impl<'c> From<&Symbol<'c>> for Value<'c> {
    fn from(value: &Symbol<'c>) -> Value<'c> {
        if value.is_quoted() {
            Value::quoted_symbol(value.quote())
        } else {
            Value::symbol(value.unquote())
        }
    }
}
impl<'c> From<&'c str> for Value<'c> {
    fn from(value: &'c str) -> Value<'c> {
        Value::String(value.to_string().leak())
    }
}
impl<'c> From<u64> for Value<'c> {
    fn from(value: u64) -> Value<'c> {
        if value < u8::MAX.into() {
            Value::Byte(value as u8)
        } else {
            Value::UnsignedInteger(value.into())
        }
    }
}
impl<'c> From<i32> for Value<'c> {
    fn from(value: i32) -> Value<'c> {
        if let Ok(value) = TryInto::<u64>::try_into(value) {
            Value::UnsignedInteger(value.into())
        } else {
            Value::Integer(value.into())
        }
    }
}

impl<'c> From<Cow<'c, str>> for Value<'c> {
    fn from(value: Cow<'c, str>) -> Value<'c> {
        Value::from(value.into_owned().to_string().leak())
    }
}
impl<'c> From<&'c mut str> for Value<'c> {
    fn from(value: &'c mut str) -> Value<'c> {
        Value::String(value.to_string().leak())
    }
}
impl<'c> From<String> for Value<'c> {
    fn from(value: String) -> Value<'c> {
        Value::String(value.leak())
    }
}
impl<'c> From<Option<String>> for Value<'c> {
    fn from(value: Option<String>) -> Value<'c> {
        match value {
            None => Value::Nil,
            Some(value) => Value::from(value),
        }
    }
}
impl<'c> From<Cell<'c>> for Value<'c> {
    fn from(cell: Cell<'c>) -> Value<'c> {
        let is_quoted = cell.is_quoted();
        let value = cell.as_value();
        if is_quoted {
            value.quote()
        } else {
            value
        }
    }
}
// impl<'c> From<List<'c>> for Value<'c> {
//     fn from(list: List<'c>) -> Value<'c> {
//         list.as_value()
//     }
// }

impl<'c> AsRef<Value<'c>> for Value<'c> {
    fn as_ref(&self) -> &Value<'c> {
        &*self
    }
}
impl<'c> AsMut<Value<'c>> for Value<'c> {
    fn as_mut(&mut self) -> &mut Value<'c> {
        &mut *self
    }
}

impl<'c> PartialEq<&Value<'c>> for Value<'c> {
    fn eq(&self, other: &&Value<'c>) -> bool {
        let other = &**other;
        self == other
    }
}
impl<'c> PartialEq<Option<Value<'c>>> for Value<'c> {
    fn eq(&self, other: &Option<Value<'c>>) -> bool {
        match other {
            Some(value) => value.eq(self),
            None => Value::Nil == self,
        }
    }
}
impl<'c> PartialEq<Cell<'c>> for Value<'c> {
    fn eq(&self, other: &Cell<'c>) -> bool {
        other.as_value() == self
    }
}
// impl<'c> PartialEq<List<'c>> for Value<'c> {
//     fn eq(&self, other: &List<'c>) -> bool {
//         other.as_value() == self
//     }
// }

impl<'c> PartialEq<&mut Value<'c>> for Value<'c> {
    fn eq(&self, other: &&mut Value<'c>) -> bool {
        let other = &**other;
        self == other
    }
}

impl<'c> AsValue<'c> for () {
    fn as_value(&self) -> Value<'c> {
        Value::Nil
    }
}
impl<'c> AsValue<'c> for bool {
    fn as_value(&self) -> Value<'c> {
        if *self {
            Value::T
        } else {
            Value::Nil
        }
    }
}
impl<'c> AsValue<'c> for u8 {
    fn as_value(&self) -> Value<'c> {
        Value::Byte(*self)
    }
}
// impl<'c> From<&'static str> for Value<'c> {
//     fn from(value: &'static str) -> Value<'c> {
//         Value::Symbol(value)
//     }
// }
impl<'c> AsValue<'c> for &'c str {
    fn as_value(&self) -> Value<'c> {
        Value::String(self.to_string().leak())
    }
}
impl<'c> AsValue<'c> for u64 {
    fn as_value(&self) -> Value<'c> {
        if *self < u8::MAX.into() {
            Value::Byte(*self as u8)
        } else {
            Value::UnsignedInteger(self.as_unsigned_integer())
        }
    }
}
impl<'c> AsValue<'c> for i32 {
    fn as_value(&self) -> Value<'c> {
        if let Ok(value) = TryInto::<u64>::try_into(*self) {
            Value::UnsignedInteger(value.as_unsigned_integer())
        } else {
            Value::Integer(self.as_integer())
        }
    }
}

impl<'c> AsValue<'c> for Cow<'c, str> {
    fn as_value(&self) -> Value<'c> {
        Value::from(self.clone().into_owned().to_string().leak())
    }
}
impl<'c> AsValue<'c> for &'c mut str {
    fn as_value(&self) -> Value<'c> {
        Value::String(self.to_string().leak())
    }
}
impl<'c> AsValue<'c> for String {
    fn as_value(&self) -> Value<'c> {
        Value::String(self.to_string().leak())
    }
}
impl<'c> AsValue<'c> for Option<String> {
    fn as_value(&self) -> Value<'c> {
        match self.clone() {
            None => Value::Nil,
            Some(value) => Value::from(value),
        }
    }
}

impl<'c> AsCell<'c> for Value<'c> {
    fn as_cell(&self) -> Cell<'c> {
        match self {
            Value::Symbol(h) => Cell::quoted(Some(h.unquote()), false),
            Value::QuotedSymbol(h) => Cell::quoted(Some(h.quote()), true),
            Value::List(h) => {
                let mut cell = Cell::nil();
                for item in h.clone().into_iter() {
                    cell.add(&Cell::new(item));
                }
                cell
            }
            Value::QuotedList(h) => {
                let mut cell = Cell::nil();
                for item in h.clone().into_iter() {
                    cell.add(&Cell::new(item));
                }
                cell.quote()
            }
            _ => Cell::new(self.clone()),
        }
    }
}
impl<'c> AsCell<'c> for &Value<'c> {
    fn as_cell(&self) -> Cell<'c> {
        let value = UniquePointer::read_only(*self).read();
        value.as_cell()
    }
}

impl<'c> ListIterator<'c, Value<'c>> for Value<'c> {
    fn iter_cells(&self) -> Cell<'c> {
        match self {
            Value::List(cell) | Value::QuotedList(cell) => cell.clone(),
            _ => Cell::from(self.clone()),
        }
    }
}
impl<'c> ListIterator<'c, Value<'c>> for &Value<'c> {
    fn iter_cells(&self) -> Cell<'c> {
        match self {
            Value::List(cell) | Value::QuotedList(cell) => cell.clone(),
            _ => Cell::from(*self),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValueIterator<'c> {
    cell: UniquePointer<Cell<'c>>,
    quoted: bool,
}

impl<'c> ValueIterator<'c> {
    pub fn new(cell: &Cell<'c>, quoted: bool) -> ValueIterator<'c> {
        ValueIterator {
            cell: UniquePointer::from_ref(cell),
            quoted,
        }
    }

    pub fn item(&self) -> Option<&Cell<'c>> {
        self.cell.as_ref()
    }

    pub fn tail(&self) -> Option<&Cell<'c>> {
        if let Some(cell) = self.cell.as_ref() {
            cell.tail()
        } else {
            None
        }
    }
}
impl<'c> Iterator for ValueIterator<'c> {
    type Item = Value<'c>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cell.is_not_null() {
            let value = self.cell.inner_ref().head();
            let next_tail = self.cell.inner_ref().tail.clone();
            self.cell = next_tail;
            value
        } else {
            None
        }
    }
}
impl<'c> Quotable for ValueIterator<'c> {
    fn is_quoted(&self) -> bool {
        self.quoted
    }

    fn set_quoted(&mut self, quoted: bool) {
        self.quoted = quoted;
    }
}
impl<'c> ValueListIterator<'c> for ValueIterator<'c> {}

impl<'c> IntoIterator for &Value<'c> {
    type IntoIter = ValueIterator<'c>;
    type Item = Value<'c>;

    fn into_iter(self) -> Self::IntoIter {
        let cell = match self.clone() {
            Value::List(ref cell) | Value::QuotedList(ref cell) => cell.clone(),
            Value::EmptyList | Value::EmptyQuotedList => Cell::nil(),
            ref value => Cell::from(value),
        };
        let cell = if self.is_quoted() { cell.quote() } else { cell };
        ValueIterator::new(&cell, self.is_quoted())
    }
}
impl<'c> IntoIterator for Value<'c> {
    type IntoIter = ValueIterator<'c>;
    type Item = Value<'c>;

    fn into_iter(self) -> Self::IntoIter {
        let cell = match &self {
            Value::List(cell) | Value::QuotedList(cell) => cell.clone(),
            Value::EmptyList | Value::EmptyQuotedList => Cell::nil(),
            value => Cell::from(value),
        };
        let cell = if self.is_quoted() { cell.quote() } else { cell };
        ValueIterator::new(&cell, self.is_quoted())
    }
}

impl<'c> FromIterator<Value<'c>> for Value<'c> {
    fn from_iter<I: IntoIterator<Item = Value<'c>>>(iter: I) -> Value<'c> {
        let mut cell = Cell::nil();
        for value in iter {
            cell.push_value(value);
        }
        Value::list(cell)
    }
}
impl<'c> Extend<Value<'c>> for Value<'c> {
    fn extend<T: IntoIterator<Item = Value<'c>>>(&mut self, iter: T) {
        if let Value::List(cell) = self {
            for value in iter {
                cell.push_value(value);
            }
        } else if let Value::QuotedList(cell) = self {
            for value in iter {
                cell.push_value(value);
            }
        } else {
            match self.clone() {
                Value::EmptyList => {
                    let mut cell = Cell::nil();
                    for value in iter {
                        cell.push_value(value)
                    }
                    *self = Value::List(cell)
                }
                Value::EmptyQuotedList => {
                    let mut cell = Cell::nil();
                    for value in iter {
                        cell.push_value(value)
                    }
                    *self = Value::QuotedList(cell)
                }
                value => {
                    let mut cell = Cell::nil();
                    for value in iter {
                        cell.push_value(value)
                    }
                    *self = Value::List(cell)
                }
            }
        }
    }
}

// impl<'c> FromIterator<Cell<'c>> for Value<'c> {
//     fn from_iter<I: IntoIterator<Item = Cell<'c>>>(iter: I) -> Value<'c> {
//         let mut cell = Cell::nil();
//         for cell in iter {
//             for value in cell {
//                 cell.push_value(value);
//             }
//         }
//         Value::list(cell)
//     }
// }

impl<'c> Quotable for Value<'c> {
    fn set_quoted(&mut self, quoted: bool) {
        match self {
            Value::Symbol(h) => {
                if quoted {
                    *self = Value::QuotedSymbol(h.clone())
                }
            }
            Value::List(h) => {
                if quoted {
                    *self = Value::QuotedList(h.clone())
                }
            }
            Value::QuotedSymbol(h) => {
                if !quoted {
                    *self = Value::Symbol(h.clone())
                }
            }
            Value::QuotedList(h) => {
                if !quoted {
                    *self = Value::List(h.clone())
                }
            }
            Value::EmptyList => {
                if quoted {
                    *self = Value::EmptyList
                }
            }
            Value::EmptyQuotedList => {
                if quoted {
                    *self = Value::EmptyQuotedList
                }
            }
            _ => {}
        }
    }

    fn is_quoted(&self) -> bool {
        match self {
            Value::Symbol(h) => false,
            Value::List(h) => false,
            Value::QuotedSymbol(h) => true,
            Value::QuotedList(h) => true,
            Value::EmptyQuotedList => true,
            _ => false,
        }
    }
}

impl<'c> AsSymbol<'c> for &Value<'c> {
    fn as_symbol(&self) -> Symbol<'c> {
        match self {
            Value::Symbol(symbol) => symbol.clone(),
            Value::QuotedSymbol(symbol) => symbol.clone(),
            value => {
                panic!("cannot convert {:#?} to symbol", self)
            }
        }
    }

    fn is_symbol(&self) -> bool {
        match self {
            Value::Symbol(symbol) | Value::QuotedSymbol(symbol) => true,
            _ => false,
        }
    }
}

impl<'c> AsSymbol<'c> for Value<'c> {
    fn as_symbol(&self) -> Symbol<'c> {
        match self {
            Value::Symbol(symbol) => symbol.clone(),
            Value::QuotedSymbol(symbol) => symbol.clone(),
            value => {
                panic!("cannot convert {:#?} to symbol", self)
            }
        }
    }

    fn is_symbol(&self) -> bool {
        match self {
            Value::Symbol(symbol) | Value::QuotedSymbol(symbol) => true,
            _ => false,
        }
    }
}

impl<'c> AsFloat for Value<'c> {
    fn as_float(&self) -> Float {
        match self {
            Value::Float(float) => *float,
            value => {
                panic!("cannot convert {:#?} to float", self)
            }
        }
    }
}

impl<'c> AsInteger for Value<'c> {
    fn as_integer(&self) -> Integer {
        match self {
            Value::Integer(integer) => *integer,
            value => {
                panic!("cannot convert {:#?} to integer", self)
            }
        }
    }
}

impl<'c> AsUnsignedInteger for Value<'c> {
    fn as_unsigned_integer(&self) -> UnsignedInteger {
        match self {
            Value::UnsignedInteger(unsigned_integer) => *unsigned_integer,
            value => {
                panic!("cannot convert {:#?} to unsigned integer", self)
            }
        }
    }
}

impl<'c> AsNumber<f64> for Value<'c> {
    fn as_number(&self) -> f64 {
        *self.as_float()
    }
}

impl<'c> AsNumber<i64> for Value<'c> {
    fn as_number(&self) -> i64 {
        *self.as_integer()
    }
}

impl<'c> AsNumber<u32> for Value<'c> {
    fn as_number(&self) -> u32 {
        *self.as_unsigned_integer()
    }
}

// impl<'c> std::cmp::PartialEq for Value<'c> {
//     fn eq(&self, rhs: &Value<'c>) -> bool {
//         match self.clone() {
//             Value::Nil => {
//                 // Value::Nil
//                 return Value::Nil == *rhs;
//             }
//             Value::T => {
//                 // Value::T
//                 return Value::T == *rhs;
//             }
//             Value::EmptyList => {
//                 // Value::EmptyList
//                 return Value::EmptyList == *rhs;
//             }
//             Value::EmptyQuotedList => {
//                 // Value::EmptyQuotedList
//                 return Value::EmptyQuotedList == *rhs;
//             }

//             Value::String(ref value) => {
//                 // Value::String
//                 if let Value::String(ref other) = rhs.clone() {
//                     return value == other;
//                 } else {
//                     return false;
//                 }
//             }
//             Value::Symbol(ref value) => {
//                 // Value::Symbol
//                 if let Value::Symbol(ref other) = rhs.clone() {
//                     return value.quote() == other.quote();
//                 } else {
//                     return false;
//                 }
//             }
//             Value::QuotedSymbol(ref value) => {
//                 // Value::QuotedSymbol
//                 if let Value::QuotedSymbol(ref other) = rhs.clone() {
//                     return value.unquote() == other.unquote();
//                 } else {
//                     return false;
//                 }
//             }
//             Value::Byte(ref value) => {
//                 // Value::Byte
//                 if let Value::Byte(ref other) = rhs.clone() {
//                     return value == other;
//                 } else {
//                     return false;
//                 }
//             }
//             Value::UnsignedInteger(ref value) => {
//                 // Value::UnsignedInteger
//                 if let Value::UnsignedInteger(ref other) = rhs.clone() {
//                     return value == other;
//                 } else {
//                     return false;
//                 }
//             }
//             Value::Integer(ref value) => {
//                 // Value::Integer
//                 if let Value::Integer(ref other) = rhs.clone() {
//                     return value == other;
//                 } else {
//                     return false;
//                 }
//             }
//             Value::Float(ref value) => {
//                 // Value::Float
//                 if let Value::Float(ref other) = rhs.clone() {
//                     return value == other;
//                 } else {
//                     return false;
//                 }
//             }
//             Value::List(ref value) => {
//                 // Value::List
//                 if let Value::List(ref other) = rhs.clone() {
//                     return value.unquote() == other.unquote();
//                 } else {
//                     return false;
//                 }
//             }
//             Value::QuotedList(ref value) => {
//                 // Value::QuotedList
//                 if let Value::QuotedList(ref other) = rhs.clone() {
//                     return value.quote() == other.quote();
//                 } else {
//                     return false;
//                 }
//             }
//         }
//         false
//     }
// }
