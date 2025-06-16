#![allow(unused)]
use std::borrow::Cow;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::iter::{Extend, IntoIterator, Iterator};
use std::ops::Deref;

use unique_pointer::{RefCounter, UniquePointer};

use crate::{AsSymbol, AsValue, Quotable, Symbol, Value};
pub trait ListIterator<'c, T: AsCell<'c>>: IntoIterator<Item = T> + Debug + Quotable {
    fn iter_cells(&self) -> Cell<'c>;
}

pub trait AsCell<'c>: Quotable {
    //: ListValue {
    fn as_cell(&self) -> Cell<'c>;
}

#[derive(Eq, PartialOrd, Ord)]
pub struct Cell<'c> {
    pub(crate) head: UniquePointer<Value<'c>>,
    pub(crate) tail: UniquePointer<Cell<'c>>,
    pub(crate) refs: RefCounter,
    pub(crate) quoted: bool,
}

impl<'c> Cell<'c> {
    pub fn nil() -> Cell<'c> {
        Cell::quoted(Option::<Value<'c>>::None, false)
    }

    pub fn quoted<T: AsValue<'c>>(item: Option<T>, quoted: bool) -> Cell<'c> {
        let mut cell = Cell {
            head: UniquePointer::<Value<'c>>::null(),
            tail: UniquePointer::<Cell<'c>>::null(),
            refs: RefCounter::new(),
            quoted,
        };
        cell.incr_ref();
        if let Some(item) = item {
            cell.write(item.as_value());
        }
        cell
    }

    pub fn is_nil(&self) -> bool {
        self.head.is_null() && self.tail.is_null()
    }

    pub fn unwrap_value(&self) -> Value<'c> {
        if self.tail.is_null() {
            match self.head() {
                Some(head) => head.unwrap_list(),
                None => Value::Nil,
            }
        } else if self.quoted {
            Value::QuotedList(self.as_cell())
        } else {
            Value::List(self.as_cell())
        }
    }

    pub fn new<T: AsValue<'c>>(item: T) -> Cell<'c> {
        let value = item.as_value();
        let is_quoted = value.is_quoted();
        Cell::quoted(Some(value), is_quoted)
        // let mut cell = Cell::nil();
        // cell.write(item.as_value());
        // cell
    }

    pub fn head(&self) -> Option<Value<'c>> {
        self.head.try_read()
    }

    pub fn push_value(&mut self, value: Value<'c>) {
        let is_quoted = value.is_quoted();
        let mut cell = Cell::quoted(Some(value), is_quoted);
        let cell = if is_quoted { cell.quote() } else { cell };
        self.add(&cell);
    }

    pub fn add(&mut self, new: &Cell<'c>) {
        if new.is_nil() {
            return;
        }
        let mut new = new.clone();
        self.incr_ref();

        if self.head.is_null() {
            // when self.head *IS* null:
            // and `new.head` *IS NOT* null
            if !new.head.is_null() {
                // swap heads
                self.swap_head(&mut new);
            }

            // and new.tail *IS NOT* null
            if new.tail.is_not_null() {
                let tail = new.tail.inner_mut();
                // if new.tail.head *IS NOT* null
                if new.head.is_not_null() {
                    // write new.tail.head in tail.head
                    tail.head.write_ref(new.head.inner_ref());

                    // TODO:
                    // try new.swap_head(tail);
                }
            }
            self.tail = UniquePointer::from(new);
        } else {
            // when self.head *IS NOT* null
            if self.tail.is_null() {
                // when self.tail *IS* null
                self.tail = UniquePointer::from(new);
            } else {
                //  self.tail *IS NOT* null
                self.tail.inner_mut().add(&new);
            }
        }
    }

    pub fn pop(&mut self) -> bool {
        if !self.tail.is_null() {
            self.tail.drop_in_place();
            self.tail = UniquePointer::null();
            true
        } else if !self.head.is_null() {
            self.head.drop_in_place();
            self.head = UniquePointer::null();
            true
        } else {
            false
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// `O(n)`
    pub fn len(&self) -> usize {
        let mut len = 0;
        if !self.head.is_null() {
            len += 1
        }
        if let Some(tail) = self.tail() {
            len += tail.len();
        }
        len
    }

    pub fn tail(&self) -> Option<&'c Cell<'c>> {
        self.tail.as_ref()
    }

    pub fn values(&self) -> Vec<Value<'c>> {
        let mut values = Vec::<Value>::new();
        if let Some(head) = self.head() {
            // dbg!(&head);
            values.push(head.clone());
        }
        if let Some(tail) = self.tail() {
            // dbg!(&tail);
            values.extend(tail.values());
        }
        values
    }

    pub(crate) fn write(&mut self, value: Value<'c>) {
        self.head.write(value);
        self.incr_ref();
    }

    pub(crate) fn swap_head(&mut self, other: &mut Self) {
        // self.incr_ref();
        // other.incr_ref();
        self.head = unsafe {
            let head = other.head.propagate();
            other.head = self.head.propagate();
            head
        };
    }

    pub(crate) fn swap_tail(&mut self, other: &mut Self) {
        self.tail = unsafe {
            let tail = other.tail.propagate();
            other.tail = self.tail.propagate();
            tail
        };
    }

    pub(crate) fn swap_refs(&mut self, other: &mut Self) {
        self.refs = {
            let refs = other.refs.clone();
            other.refs = self.refs.clone();
            refs
        };
    }

    pub fn to_vec(&self) -> Vec<Value<'c>> {
        Vec::<Value<'c>>::from_iter(self.clone().into_iter())
    }

    fn incr_ref(&mut self) {
        self.refs.incr();
        if !self.tail.is_null() {
            if let Some(tail) = self.tail.as_mut() {
                tail.incr_ref();
            }
        }
    }

    fn decr_ref(&mut self) {
        self.refs.decr();
        if !self.tail.is_null() {
            if let Some(tail) = self.tail.as_mut() {
                tail.decr_ref();
            }
        }
    }

    fn dealloc(&mut self) {
        if self.refs > 0 {
            self.decr_ref();
        } else {
            self.head.drop_in_place();
            self.tail.drop_in_place();
        }
    }

    fn repr(&self) -> String {
        [
            "Cell".to_string(),
            format!(
                "[{}]",
                if self.is_nil() {
                    format!("null")
                } else {
                    [
                        if self.head.is_null() {
                            format!("head: {}", "null")
                        } else {
                            format!("head={:#?}", self.head().unwrap_or_default())
                        },
                        if self.tail.is_null() {
                            format!("tail: {}", "null")
                        } else {
                            format!(
                                "tail={:#?}",
                                self.tail().map(Clone::clone).unwrap_or_default()
                            )
                        },
                    ]
                    .join(" | ")
                }
            ),
        ]
        .join("")
    }
}
impl<'c> Quotable for Cell<'c> {
    fn is_quoted(&self) -> bool {
        self.quoted
    }

    fn set_quoted(&mut self, quoted: bool) {
        self.quoted = quoted;
    }
}

impl<'c, T: Quotable + AsCell<'c>, const N: usize> AsCell<'c> for [T; N] {
    fn as_cell(&self) -> Cell<'c> {
        let mut cell = Cell::nil();
        for item in self {
            cell.add(&item.as_cell());
        }
        cell
    }
}

// impl<'c> AsList<'c> for Cell<'c> {
//     fn as_cell(&self) -> List<'c> {
//         if self.is_nil() {
//             List::Empty(self.is_quoted())
//         } else {
//             List::Linked(self.clone(), self.is_quoted())
//         }
//     }
// }

impl<'c, T: AsCell<'c>, const N: usize> ListIterator<'c, T> for [T; N] {
    fn iter_cells(&self) -> Cell<'c> {
        let mut cell = Cell::nil();
        for item in self {
            cell.add(&item.as_cell());
        }
        cell
    }
}
impl<'c> ListIterator<'c, Value<'c>> for Cell<'c> {
    fn iter_cells(&self) -> Cell<'c> {
        self.clone()
    }
}
impl<'c> AsCell<'c> for Cell<'c> {
    fn as_cell(&self) -> Cell<'c> {
        self.clone()
    }
}
impl<'c> AsCell<'c> for &Cell<'c> {
    fn as_cell(&self) -> Cell<'c> {
        UniquePointer::read_only(*self).read()
    }
}

impl<'c> AsCell<'c> for &'c str {
    fn as_cell(&self) -> Cell<'c> {
        Cell::new(Value::symbol(self.to_string()))
    }
}
impl<'c> AsCell<'c> for String {
    fn as_cell(&self) -> Cell<'c> {
        Cell::new(Value::string(self))
    }
}

impl<'c> From<Value<'c>> for Cell<'c> {
    fn from(value: Value<'c>) -> Cell<'c> {
        Cell::quoted(Some(value.clone()), value.is_quoted())
    }
}
impl<'c> From<&Value<'c>> for Cell<'c> {
    fn from(value: &Value<'c>) -> Cell<'c> {
        Cell::quoted(Some(value.clone()), value.is_quoted())
    }
}

impl<'c> From<u8> for Cell<'c> {
    fn from(value: u8) -> Cell<'c> {
        Cell::new(Value::Byte(value))
    }
}
impl<'c> From<u32> for Cell<'c> {
    fn from(value: u32) -> Cell<'c> {
        if value <= u8::MAX.into() {
            Cell::new(Value::Byte(value as u8))
        } else {
            Cell::new(Value::UnsignedInteger(value.into()))
        }
    }
}
impl<'c> From<f64> for Cell<'c> {
    fn from(value: f64) -> Cell<'c> {
        Cell::new(Value::float(value))
    }
}
impl<'c> From<u64> for Cell<'c> {
    fn from(value: u64) -> Cell<'c> {
        if value <= u32::MAX.into() {
            Cell::from(value as u32)
        } else {
            Cell::new(Value::UnsignedInteger(value.into()))
        }
    }
}
impl<'c> From<i32> for Cell<'c> {
    fn from(value: i32) -> Cell<'c> {
        if let Ok(value) = TryInto::<u32>::try_into(value) {
            Cell::new(Value::unsigned_integer(value))
        } else {
            Cell::new(Value::integer(value))
        }
    }
}
impl<'c> From<i64> for Cell<'c> {
    fn from(value: i64) -> Cell<'c> {
        Cell::new(Value::from(value))
    }
}
impl<'c> From<&str> for Cell<'c> {
    fn from(value: &str) -> Cell<'c> {
        Cell::new(Value::symbol(value))
    }
}
impl<'c> From<String> for Cell<'c> {
    fn from(value: String) -> Cell<'c> {
        Cell::new(Value::string(value))
    }
}
impl<'c> From<Cow<'c, str>> for Cell<'c> {
    fn from(value: Cow<'c, str>) -> Cell<'c> {
        Cell::new(Value::string(&value))
    }
}

impl<'c> PartialEq<Cell<'c>> for Cell<'c> {
    fn eq(&self, other: &Cell<'c>) -> bool {
        if self.is_nil() && other.is_nil() {
            return true;
        }
        let slen = self.len();
        let olen = other.len();
        if slen != olen {
            return false;
        }

        let max_len = std::cmp::max(slen, olen);
        let mut current = 0;

        let mut iter = self.clone().into_iter().zip(other.clone().into_iter());
        loop {
            current += 1;
            match iter.next() {
                Some((lhs, rhs)) => {
                    if lhs != rhs {
                        return false;
                    }
                }
                None => return current < max_len,
            }
            if current == max_len {
                break true;
            }
        }
    }
}

impl<'c> Default for Cell<'c> {
    fn default() -> Cell<'c> {
        Cell::nil()
    }
}

/// [`Clone`] implementation for [`Cell`] clones the internal
/// pointers.
impl<'c> Clone for Cell<'c> {
    fn clone(&self) -> Cell<'c> {
        let mut cell = Cell::nil();
        cell.refs = self.refs.clone();
        cell.incr_ref();
        if let Some(head) = self.head() {
            cell.head.write(head)
        }
        if let Some(tail) = self.tail().map(Clone::clone) {
            cell.tail.write(tail)
        }
        cell
    }
}
impl<'c> Hash for Cell<'c> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.head().hash(state);
        self.tail().hash(state);
        self.refs.hash(state);
        self.quoted.hash(state);
    }
}
impl<'c> Drop for Cell<'c> {
    fn drop(&mut self) {
        self.dealloc();
    }
}

impl std::fmt::Debug for Cell<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_nil() {
            return write!(f, "nil");
        }
        write!(
            f,
            "Cell[{}]",
            if self.is_nil() {
                "".to_string()
            } else {
                let mut parts = Vec::<String>::new();
                if self.head.is_not_null() {
                    parts.push(
                        self.head()
                            .map(|value| format!("{:#?}", value))
                            .unwrap_or_default(),
                    )
                }

                if self.tail.is_not_null() {
                    if let Some(tail) = self.tail() {
                        parts.push(format!("{:#?}", tail));
                    }
                }
                parts.join(" . ").trim().to_string()
            }
        )
    }
}

impl std::fmt::Display for Cell<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if self.is_nil() {
                "".to_string()
            } else {
                let mut parts = Vec::<String>::new();
                if self.head.is_not_null() {
                    parts.push(
                        self.head()
                            .map(|value| value.to_string())
                            .unwrap_or_default(),
                    )
                }

                if self.tail.is_not_null() {
                    if let Some(tail) = self.tail() {
                        parts.push(tail.to_string());
                    }
                }
                parts.join(" ").trim().to_string()
            }
        )
    }
}

impl<'c> AsValue<'c> for Cell<'c> {
    fn as_value(&self) -> Value<'c> {
        if self.tail.is_null() {
            match self.head() {
                Some(head) => {
                    let is_quoted = head.is_quoted();
                    let value = head.unwrap_list();
                    if is_quoted {
                        value.quote()
                    } else {
                        value
                    }
                }
                None => Value::Nil,
            }
        } else if self.quoted {
            Value::QuotedList(self.clone())
        } else {
            Value::List(self.clone())
        }
    }
}

pub struct CellIterator<'c> {
    cell: UniquePointer<Cell<'c>>,
}

impl<'c> CellIterator<'c> {
    pub fn new(cell: Cell<'c>) -> CellIterator<'c> {
        CellIterator {
            cell: UniquePointer::from_ref(&cell),
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
impl<'c> Iterator for CellIterator<'c> {
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

impl<'c> IntoIterator for Cell<'c> {
    type IntoIter = CellIterator<'c>;
    type Item = Value<'c>;

    fn into_iter(self) -> Self::IntoIter {
        CellIterator::new(self)
    }
}

impl<'c> FromIterator<Value<'c>> for Cell<'c> {
    fn from_iter<I: IntoIterator<Item = Value<'c>>>(iter: I) -> Cell<'c> {
        let mut cell = Cell::nil();
        for value in iter {
            cell.push_value(value);
        }
        cell
    }
}
// impl<'c> Extend<Value<'c>> for Value<'c> {
//     fn extend<T: IntoIterator<Item = Value<'c>>>(&mut self, iter: T) {
//         if let Value::List(ref mut cell) = self {
//             for value in iter {
//                 cell.push_value(value);
//             }
//         } else if let Value::QuotedList(ref mut cell) = self {
//             for value in iter {
//                 cell.push_value(value);
//             }
//         } else {
//             match self.clone() {
//                 Value::EmptyList => {
//                     let mut cell = Cell::nil();
//                     for value in iter {
//                         cell.push_value(value)
//                     }
//                     *self = Value::List(cell)
//                 },
//                 Value::EmptyQuotedList => {
//                     let mut cell = Cell::nil();
//                     for value in iter {
//                         cell.push_value(value)
//                     }
//                     *self = Value::QuotedList(cell)
//                 },
//                 value => {
//                     let mut cell = Cell::nil();
//                     for value in iter {
//                         cell.push_value(value)
//                     }
//                     *self = Value::List(cell)
//                 },
//             }
//         }
//     }
// }
