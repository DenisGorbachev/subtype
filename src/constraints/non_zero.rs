#![cfg(feature = "num-traits")]

use crate::traits::check::Check;
use crate::{impl_transform_as_validate, impl_validate_as_check};
use num_traits::Zero;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct NonZero;

impl<V: Zero> Check<V> for NonZero {
    fn check(value: &V) -> bool {
        !value.is_zero()
    }
}

impl_validate_as_check!(NonZero, <>, where Value: Zero);

impl_transform_as_validate!(NonZero, <>, where Value: Zero);
