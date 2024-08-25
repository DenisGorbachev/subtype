use crate::Conjure;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct I32<const VALUE: i32>;

impl<const VALUE: i32> From<I32<VALUE>> for i32 {
    fn from(_v: I32<VALUE>) -> Self {
        VALUE
    }
}

impl<const VALUE: i32> From<I32<VALUE>> for i64 {
    fn from(_v: I32<VALUE>) -> Self {
        VALUE as i64
    }
}

impl<const VALUE: i32> From<I32<VALUE>> for i128 {
    fn from(_v: I32<VALUE>) -> Self {
        VALUE as i128
    }
}

impl<const VALUE: i32> Conjure<i32> for I32<VALUE> {
    fn conjure() -> i32 {
        VALUE
    }
}

impl<const VALUE: i32> Conjure<i64> for I32<VALUE> {
    fn conjure() -> i64 {
        VALUE as i64
    }
}

impl<const VALUE: i32> Conjure<i128> for I32<VALUE> {
    fn conjure() -> i128 {
        VALUE as i128
    }
}
