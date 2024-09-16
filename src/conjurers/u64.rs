use crate::Conjure;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct U64<const VALUE: u64>;

impl<const VALUE: u64> From<U64<VALUE>> for u64 {
    fn from(_v: U64<VALUE>) -> Self {
        VALUE
    }
}

impl<const VALUE: u64> From<U64<VALUE>> for u128 {
    fn from(_v: U64<VALUE>) -> Self {
        VALUE as u128
    }
}

impl<const VALUE: u64> Conjure<u64> for U64<VALUE> {
    fn conjure() -> u64 {
        VALUE
    }
}

impl<const VALUE: u64> Conjure<u128> for U64<VALUE> {
    fn conjure() -> u128 {
        VALUE as u128
    }
}
