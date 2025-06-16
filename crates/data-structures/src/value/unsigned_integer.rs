use crate::impl_number_type;
impl_number_type!(
    u32,
    UnsignedInteger,
    AsUnsignedInteger,
    as_unsigned_integer
);

impl From<u64> for UnsignedInteger {
    fn from(value: u64) -> UnsignedInteger {
        if value <= u32::MAX.into() {
            UnsignedInteger {
                value: value as u32,
            }
        } else {
            panic!(
                "cannot convert from {:#?} to {}",
                value,
                UnsignedInteger::type_name()
            )
        }
    }
}

impl AsNumber<u32> for u64 {
    fn as_number(&self) -> u32 {
        if *self <= u32::MAX.into() {
            *self as u32
        } else {
            panic!(
                "cannot convert from {:#?} to u32",
                self,
            )
        }
    }
}

impl AsNumber<u32> for &u64 {
    fn as_number(&self) -> u32 {
        let value = **self;
        if value <= u32::MAX.into() {
            value as u32
        } else {
            panic!(
                "cannot convert from {:#?} to u32",
                value,
            )
        }
    }
}

impl AsUnsignedInteger for u64 {
    fn as_unsigned_integer(&self) -> UnsignedInteger {
        UnsignedInteger::from(*self)
    }
}

impl AsUnsignedInteger for &u64 {
    fn as_unsigned_integer(&self) -> UnsignedInteger {
        UnsignedInteger::from(**self)
    }
}
