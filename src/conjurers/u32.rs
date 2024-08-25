use crate::Conjure;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct U32<const VALUE: u32>;

impl<const VALUE: u32> From<U32<VALUE>> for u32 {
    fn from(_v: U32<VALUE>) -> Self {
        VALUE
    }
}

impl<const VALUE: u32> From<U32<VALUE>> for u64 {
    fn from(_v: U32<VALUE>) -> Self {
        VALUE as u64
    }
}

impl<const VALUE: u32> From<U32<VALUE>> for u128 {
    fn from(_v: U32<VALUE>) -> Self {
        VALUE as u128
    }
}

impl<const VALUE: u32> Conjure<u32> for U32<VALUE> {
    fn conjure() -> u32 {
        VALUE
    }
}

impl<const VALUE: u32> Conjure<u64> for U32<VALUE> {
    fn conjure() -> u64 {
        VALUE as u64
    }
}

impl<const VALUE: u32> Conjure<u128> for U32<VALUE> {
    fn conjure() -> u128 {
        VALUE as u128
    }
}
