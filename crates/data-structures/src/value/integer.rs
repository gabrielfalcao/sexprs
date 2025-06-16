use crate::impl_number_type;
impl_number_type!(i64, Integer, AsInteger, as_integer);

impl From<i32> for Integer {
    fn from(value: i32) -> Integer {
        if let Ok(value) = TryInto::<u32>::try_into(value) {
            Integer{value: value.into()}
        } else {
            panic!("cannot convert from {:#?} to {}", value, Integer::type_name())
        }
    }
}

impl AsNumber<i64> for i32 {
    fn as_number(&self) -> i64 {
        *self as i64
    }
}

impl AsInteger for i32 {
    fn as_integer(&self) -> Integer {
        Integer::from(*self as i64)
    }
}
