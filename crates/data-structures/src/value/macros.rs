#[macro_export]
macro_rules! impl_number_type {
    ($type:ty, $name:ident, $trait:ident, $method_name:ident) => {
        use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
        use std::convert::{AsMut, AsRef};
        use std::fmt::{Debug, Display, Formatter};
        use std::hash::{Hash, Hasher};
        use std::ops::{Add, Deref, DerefMut, Div, Mul, Sub};

        use crate::AsNumber;

        pub trait $trait: AsNumber<$type> {
            fn inner(&self) -> $type {
                self.as_number()
            }
            fn $method_name(&self) -> $name;
        }

        #[derive(Clone, Copy)]
        pub struct $name {
            value: $type,
        }

        impl $name {
            pub fn to_bytes(&self) -> Vec<u8> {
                let mut prefix = [0u8; 4];
                let type_bytes =
                    $crate::$name::type_name().as_bytes().to_vec();
                if type_bytes.len() == 4 {
                    prefix.copy_from_slice(&type_bytes[..]);
                } else {
                    prefix[4 - type_bytes.len()..]
                        .copy_from_slice(&type_bytes[..]);
                }
                let mut prefix = prefix.to_vec();
                prefix.extend(self.inner().to_be_bytes());
                prefix
            }

            pub fn type_name() -> &'static str {
                std::any::type_name::<$type>()
            }
        }
        impl Into<$type> for $name {
            fn into(self) -> $type {
                self.inner()
            }
        }
        impl From<$type> for $name {
            fn from(value: $type) -> $name {
                $crate::$name { value: value }
            }
        }
        impl From<&$type> for $name {
            fn from(value: &$type) -> $name {
                $crate::$name { value: *value }
            }
        }
        impl<'c> From<$type> for $crate::Value<'c> {
            fn from(value: $type) -> $crate::Value<'c> {
                $crate::Value::$name($crate::$name { value: value })
            }
        }
        impl<'c> From<&$type> for $crate::Value<'c> {
            fn from(value: &$type) -> $crate::Value<'c> {
                $crate::Value::$name($crate::$name { value: *value })
            }
        }

        impl AsRef<$type> for $name {
            fn as_ref(&self) -> &$type {
                &self.value
            }
        }
        impl AsMut<$type> for $name {
            fn as_mut(&mut self) -> &mut $type {
                &mut self.value
            }
        }

        impl Deref for $name {
            type Target = $type;

            fn deref(&self) -> &$type {
                &self.value
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut $type {
                &mut self.value
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &$name) -> bool {
                self.to_bytes().eq(&other.to_bytes())
            }
        }
        impl Eq for $name {}
        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &$name) -> Option<Ordering> {
                self.to_bytes().partial_cmp(&other.to_bytes())
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> Ordering {
                self.to_bytes().cmp(&other.to_bytes())
            }
        }

        impl Hash for $name {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.to_bytes().hash(state)
            }
        }
        impl $crate::AsNumber<$type> for &$type {
            fn as_number(&self) -> $type {
                **self
            }
        }
        impl $crate::AsNumber<$type> for $name {
            fn as_number(&self) -> $type {
                self.value
            }
        }
        impl $trait for &$type {
            fn $method_name(&self) -> $name {
                $crate::$name { value: **self }
            }
        }
        impl $trait for $type {
            fn $method_name(&self) -> $name {
                $crate::$name { value: *self }
            }
        }
        impl $trait for $name {
            fn $method_name(&self) -> $name {
                self.clone()
            }
        }
        impl AsNumber<$type> for &$name {
            fn as_number(&self) -> $type {
                self.value
            }
        }
        impl $trait for &$name {
            fn $method_name(&self) -> $name {
                (*self).clone()
            }
        }
        impl Add for $name {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                $crate::$name {
                    value: self.inner().add(other.inner()),
                }
            }
        }
        impl Sub for $name {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                $crate::$name {
                    value: self.inner().sub(other.inner()),
                }
            }
        }
        impl Mul for $name {
            type Output = Self;

            fn mul(self, other: Self) -> Self {
                $crate::$name {
                    value: self.inner().mul(other.inner()),
                }
            }
        }
        impl Div for $name {
            type Output = Self;

            fn div(self, other: Self) -> Self {
                $crate::$name {
                    value: self.inner().div(other.inner()),
                }
            }
        }
    };
}
