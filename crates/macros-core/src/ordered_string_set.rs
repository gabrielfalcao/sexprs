use std::iter::{Extend, FromIterator, IntoIterator};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::ptr::NonNull;

/// `OrderedStringSet` is a safe data-structure that provides an
/// ordered set of strings with push/insert methods to help migrate
/// from both [`std::vec::Vec`] and [`std::collections::BTreeSet`]
/// relatively easily
#[derive(Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
pub struct OrderedStringSet {
    data: Vec<String>,
}
impl OrderedStringSet {
    /// `new`
    ///
    /// ```
    /// use sexprs_macros_core::OrderedStringSet;
    /// let mut set = OrderedStringSet::new();
    /// set.add(format!("a"));
    /// assert_eq!(set, vec!["a"]);
    /// ```
    pub fn new() -> OrderedStringSet {
        OrderedStringSet::default()
    }

    /// `contains`
    ///
    /// ```
    /// use sexprs_macros_core::OrderedStringSet;
    ///
    /// let mut set = OrderedStringSet::new();
    /// set.add(String::from("a"));
    ///
    /// assert!(set.contains("a"));
    /// assert!(set.contains(String::from("a")));
    /// assert!(set.contains(&String::from("a")));
    /// ```
    pub fn contains<T: std::fmt::Display>(&mut self, string: T) -> bool {
        let string = string.to_string();
        self.data.contains(&string)
    }

    /// `add`
    ///
    /// ```
    /// use sexprs_macros_core::OrderedStringSet;
    ///
    /// let mut set = OrderedStringSet::new();
    /// set.add("a");
    /// set.add(format!("a"));
    /// set.add(String::from("A").to_lowercase());
    /// assert_eq!(set, vec!["a"]);
    /// ```
    pub fn add<T: std::fmt::Display>(&mut self, string: T) {
        let string = string.to_string();
        if !self.data.contains(&string) {
            self.data.push(string.to_string())
        }
    }

    /// `first`
    ///
    /// ```
    /// use sexprs_macros_core::OrderedStringSet;
    ///
    /// let set = OrderedStringSet::from(["a", "z"]);
    ///
    /// assert_eq!(set.first(), Some(&"a".to_string()));
    /// ```
    pub fn first(&self) -> Option<&String> {
        self.data.first()
    }

    /// `last`
    ///
    /// ```
    /// use sexprs_macros_core::OrderedStringSet;
    ///
    /// let set = OrderedStringSet::from(["a", "z"]);
    ///
    /// assert_eq!(set.last(), Some(&"z".to_string()));
    /// ```
    pub fn last(&self) -> Option<&String> {
        self.data.last()
    }

    /// `push` - [`std::vec::Vec`] migration interface
    ///
    /// ```
    /// use sexprs_macros_core::OrderedStringSet;
    ///
    /// let mut set = OrderedStringSet::new();
    /// set.push("a");
    /// set.push(format!("a"));
    /// set.push(String::from("A").to_lowercase());
    /// assert_eq!(set, vec!["a"]);
    /// ```
    pub fn push<T: std::fmt::Display>(&mut self, string: T) {
        self.add(string)
    }

    /// `insert` - [`std::collections::BTreeSet`] migration interface
    ///
    /// ```
    /// use sexprs_macros_core::OrderedStringSet;
    ///
    /// let mut set = OrderedStringSet::new();
    /// set.insert("a");
    /// set.insert(format!("a"));
    /// set.insert(String::from("A").to_lowercase());
    /// assert_eq!(set, vec!["a"]);
    /// ```
    pub fn insert<T: std::fmt::Display>(&mut self, string: T) {
        self.push(string)
    }

    /// `len`
    ///
    /// ```
    /// use sexprs_macros_core::OrderedStringSet;
    ///
    /// let mut set = OrderedStringSet::from(&["a", "b", "c"]);
    /// assert_eq!(set.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// `iter_mut`
    ///
    /// ```
    /// use sexprs_macros_core::OrderedStringSet;
    ///
    /// let mut set = OrderedStringSet::new();
    ///
    /// set.push("z");
    /// set.push("c");
    /// set.push("a");
    /// set.push("b");
    ///
    /// set.extend(&["a", "b", "c", "d"]);
    ///
    /// for h in set.iter_mut() {
    ///     if !["c", "d"].contains(&h.to_lowercase().as_str()) {
    ///         *h = h.to_uppercase();
    ///     }
    /// }
    ///
    /// assert_eq!(set, vec!["Z", "c", "A", "B", "d"]);
    /// ```
    pub fn iter_mut(&mut self) -> OrderedStringSetIterMut {
        OrderedStringSetIterMut::new(&mut self.data)
    }

    /// `iter`
    ///
    /// ```
    /// use sexprs_macros_core::OrderedStringSet;
    ///
    /// let mut set = OrderedStringSet::new();
    ///
    /// set.push("z");
    /// set.push("c");
    /// set.push("a");
    /// set.push("b");
    ///
    /// set.extend(&["a", "b", "c", "d"]);
    ///
    /// let mut data = OrderedStringSet::new();
    /// for h in set {
    ///     if ["c", "d"].contains(&h.to_lowercase().as_str()) {
    ///         data.push(h.to_uppercase());
    ///     }
    /// }
    ///
    /// assert_eq!(data, vec!["C", "D"]);
    /// ```
    pub fn iter(&self) -> OrderedStringSetIter {
        OrderedStringSetIter::new(&self.data)
    }
}

/// mutable [`std::iter::Iterator`] for [`OrderedStringSet`]
pub struct OrderedStringSetIterMut<'a> {
    ptr: NonNull<String>,
    end_or_len: *mut String,
    _marker: PhantomData<&'a String>,
}
impl<'a> OrderedStringSetIterMut<'a> {
    pub fn new(slice: &mut [String]) -> Self {
        let len = slice.len();
        let ptr: NonNull<String> = NonNull::from_ref(slice).cast();
        unsafe {
            let end_or_len = ptr.as_ptr().add(len);

            Self {
                ptr,
                end_or_len,
                _marker: PhantomData,
            }
        }
    }
}
impl<'a> Iterator for OrderedStringSetIterMut<'a> {
    type Item = &'a mut String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr.addr().get() == self.end_or_len.addr() {
            None
        } else {
            unsafe {
                let item = self.ptr.as_mut();
                let ptr = self.ptr.add(1);
                self.ptr = ptr;
                Some(item)
            }
        }
    }
}
/// [`std::iter::Iterator`] for [`OrderedStringSet`]
pub struct OrderedStringSetIter<'a> {
    ptr: NonNull<String>,
    end_or_len: *const String,
    _marker: PhantomData<&'a String>,
}
impl<'a> OrderedStringSetIter<'a> {
    pub fn new(slice: &'a [String]) -> Self {
        let len = slice.len();
        let ptr: NonNull<String> = NonNull::from_ref(slice).cast();
        unsafe {
            let end_or_len = ptr.as_ptr().add(len);
            Self {
                ptr,
                end_or_len,
                _marker: PhantomData,
            }
        }
    }
}
impl<'a> Iterator for OrderedStringSetIter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr.addr().get() == self.end_or_len.addr() {
            None
        } else {
            unsafe {
                let item = self.ptr.as_ref();
                let ptr = self.ptr.add(1);
                self.ptr = ptr;
                Some(item)
            }
        }
    }
}

impl<'a, 'b> Extend<&'a &'b String> for OrderedStringSet {
    fn extend<T: IntoIterator<Item = &'a &'b String>>(&mut self, iter: T) {
        for string in iter {
            let string = string.to_string();
            if !self.contains(&string) {
                self.push(string);
            }
        }
    }
}
impl<'a, 'b> Extend<&'a &'b str> for OrderedStringSet {
    fn extend<T: IntoIterator<Item = &'a &'b str>>(&mut self, iter: T) {
        for string in iter {
            let string = string.to_string();
            if !self.contains(&string) {
                self.push(string);
            }
        }
    }
}
impl<'a> Extend<&'a str> for OrderedStringSet {
    fn extend<T: IntoIterator<Item = &'a str>>(&mut self, iter: T) {
        for string in iter {
            let string = string.to_string();
            if !self.contains(&string) {
                self.push(string);
            }
        }
    }
}

impl<'a> Extend<&'a String> for OrderedStringSet {
    fn extend<T: IntoIterator<Item = &'a String>>(&mut self, iter: T) {
        for string in iter {
            if !self.contains(string) {
                self.push(string.to_string());
            }
        }
    }
}
impl Extend<String> for OrderedStringSet {
    fn extend<T: IntoIterator<Item = String>>(&mut self, iter: T) {
        for string in iter {
            if !self.contains(&string) {
                self.push(string);
            }
        }
    }
}
impl<'a> Extend<&'a OrderedStringSet> for OrderedStringSet {
    fn extend<T: IntoIterator<Item = &'a OrderedStringSet>>(&mut self, iter: T) {
        for set in iter {
            self.extend(set.iter());
        }
    }
}
impl IntoIterator for OrderedStringSet {
    type IntoIter = std::vec::IntoIter<String>;
    type Item = String;

    fn into_iter(self) -> Self::IntoIter {
        self.data.clone().into_iter()
    }
}

impl<'a> IntoIterator for &'a OrderedStringSet {
    type IntoIter = OrderedStringSetIter<'a>;
    type Item = &'a String;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl From<Vec<String>> for OrderedStringSet {
    fn from(iter: Vec<String>) -> OrderedStringSet {
        let mut buf = OrderedStringSet::new();
        buf.extend(iter);
        buf
    }
}
impl<'a, const N: usize> From<[&'a str; N]> for OrderedStringSet {
    fn from(iter: [&'a str; N]) -> OrderedStringSet {
        let mut buf = OrderedStringSet::new();
        buf.extend(iter);
        buf
    }
}
impl<'a, 'b> From<&'a [&'b str]> for OrderedStringSet {
    fn from(iter: &'a [&'b str]) -> OrderedStringSet {
        let mut buf = OrderedStringSet::new();
        buf.extend(iter);
        buf
    }
}
impl<'a, 'b, const N: usize> From<&'a [&'b str; N]> for OrderedStringSet {
    fn from(iter: &'a [&'b str; N]) -> OrderedStringSet {
        let mut buf = OrderedStringSet::new();
        buf.extend(iter);
        buf
    }
}
impl<const N: usize> From<[String; N]> for OrderedStringSet {
    fn from(iter: [String; N]) -> OrderedStringSet {
        let mut buf = OrderedStringSet::new();
        buf.extend(iter);
        buf
    }
}
impl<'a, const N: usize> From<[&'a String; N]> for OrderedStringSet {
    fn from(iter: [&'a String; N]) -> OrderedStringSet {
        let mut buf = OrderedStringSet::new();
        buf.extend(iter);
        buf
    }
}

impl<'a, 'b> From<&'a [&'b String]> for OrderedStringSet {
    fn from(iter: &'a [&'b String]) -> OrderedStringSet {
        let mut buf = OrderedStringSet::new();
        buf.extend(iter);
        buf
    }
}
impl<'a, 'b, const N: usize> From<&'a [&'b String; N]> for OrderedStringSet {
    fn from(iter: &'a [&'b String; N]) -> OrderedStringSet {
        let mut buf = OrderedStringSet::new();
        buf.extend(iter);
        buf
    }
}

impl FromIterator<String> for OrderedStringSet {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> OrderedStringSet {
        let mut buf = OrderedStringSet::new();
        buf.extend(iter);
        buf
    }
}

impl<'a> FromIterator<&'a OrderedStringSet> for OrderedStringSet {
    fn from_iter<I: IntoIterator<Item = &'a OrderedStringSet>>(iter: I) -> OrderedStringSet {
        let mut buf = OrderedStringSet::new();
        for set in iter {
            buf.extend(set.iter());
        }
        buf
    }
}
impl FromIterator<OrderedStringSet> for OrderedStringSet {
    fn from_iter<I: IntoIterator<Item = OrderedStringSet>>(iter: I) -> OrderedStringSet {
        let mut buf = OrderedStringSet::new();
        for set in iter {
            buf.extend(set.iter());
        }
        buf
    }
}
impl<'a> FromIterator<&'a String> for OrderedStringSet {
    fn from_iter<I: IntoIterator<Item = &'a String>>(iter: I) -> OrderedStringSet {
        let mut buf = OrderedStringSet::new();
        buf.extend(iter.into_iter().map(String::from).collect::<Vec<String>>());
        buf
    }
}

impl Index<usize> for OrderedStringSet {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        Index::index(&self.data, index)
    }
}

impl IndexMut<usize> for OrderedStringSet {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        IndexMut::index_mut(&mut self.data, index)
    }
}

impl Deref for OrderedStringSet {
    type Target = [String];

    fn deref(&self) -> &[String] {
        self.data.as_slice()
    }
}

impl DerefMut for OrderedStringSet {
    fn deref_mut(&mut self) -> &mut [String] {
        self.data.as_mut_slice()
    }
}

impl std::fmt::Debug for OrderedStringSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:#?}", &self.data)
    }
}

impl<'a, const N: usize> PartialEq<[&'a str; N]> for OrderedStringSet {
    fn eq(&self, other: &[&'a str; N]) -> bool {
        let count = self.len();
        if count != N {
            return false;
        }
        let mut index = 0;
        while index < N {
            if self[index].to_string() != other[index].to_string() {
                return false;
            }
            index += 1;
        }
        true
    }
}

impl PartialEq<Vec<&str>> for OrderedStringSet {
    fn eq(&self, other: &Vec<&str>) -> bool {
        let count = self.len();
        if count != other.len() {
            return false;
        }
        let mut index = 0;
        while index < count {
            if self[index].to_string() != other[index].to_string() {
                return false;
            }
            index += 1;
        }
        true
    }
}

impl PartialEq<Vec<String>> for OrderedStringSet {
    fn eq(&self, other: &Vec<String>) -> bool {
        let count = self.len();
        if count != other.len() {
            return false;
        }
        let mut index = 0;
        while index < count {
            if self[index].to_string() != other[index].to_string() {
                return false;
            }
            index += 1;
        }
        true
    }
}

/// `oss!` is a drop-in replacement for the [`vec!`] macro.
///
/// # Example
///
/// ```
/// use sexprs_macros_core::oss;
///
/// let set = oss!["z", "a", "y", "a", "x", "c", "a", "z", "c" ];
/// assert_eq!(set, vec!["z", "a", "y", "x", "c"]);
/// ```

#[rustfmt::skip]
#[macro_export]
macro_rules! oss {
    ($( $arg:expr ),* ) => {{
        let mut set = $crate::OrderedStringSet::new();
        $(
            set.push($arg);
        )*
        set
    }};
}
