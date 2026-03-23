use crate::Conjure;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct I64<const VALUE: i64>;

impl<const VALUE: i64> From<I64<VALUE>> for i64 {
    fn from(_v: I64<VALUE>) -> Self {
        VALUE
    }
}

impl<const VALUE: i64> From<I64<VALUE>> for i128 {
    fn from(_v: I64<VALUE>) -> Self {
        VALUE as i128
    }
}

impl<const VALUE: i64> Conjure<i64> for I64<VALUE> {
    fn conjure() -> i64 {
        VALUE
    }
}

impl<const VALUE: i64> Conjure<i128> for I64<VALUE> {
    fn conjure() -> i128 {
        VALUE as i128
    }
}
