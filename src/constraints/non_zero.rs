#![cfg(feature = "num-traits")]

use derive_more::Error;
use num_traits::Zero;

use crate::traits::check::Check;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct NonZero;

#[derive(Error, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct NonZeroError<V> {
    value: V,
}

impl<V: Zero> Check<V> for NonZero {
    fn check(value: &V) -> bool {
        !value.is_zero()
    }
}
