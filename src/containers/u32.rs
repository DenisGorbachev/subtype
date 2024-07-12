use crate::traits::value::Value;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct U32<const VALUE: u32>;

impl<const VALUE: u32> From<U32<VALUE>> for u32 {
    fn from(_v: U32<VALUE>) -> Self {
        VALUE
    }
}

impl<const VALUE: u32> Value<u32> for U32<VALUE> {
    fn value() -> u32 {
        VALUE
    }
}

impl<const VALUE: u32> Value<u64> for U32<VALUE> {
    fn value() -> u64 {
        VALUE as u64
    }
}

impl<const VALUE: u32> Value<u128> for U32<VALUE> {
    fn value() -> u128 {
        VALUE as u128
    }
}
