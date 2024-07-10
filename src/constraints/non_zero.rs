#![cfg(feature = "num-traits")]

use crate::traits::check::Check;
use crate::{transform_as_validate, validate_as_check};
use num_traits::Zero;
use std::fmt::Debug;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct NonZero;

impl<V: Zero> Check<V> for NonZero {
    fn check(value: &V) -> bool {
        !value.is_zero()
    }
}

validate_as_check!(impl<V> Validate<V> for NonZero where V: Zero);

transform_as_validate!(impl<V> Transform<V> for NonZero where V: Zero);
