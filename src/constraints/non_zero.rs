#![cfg(feature = "num-traits")]

use std::fmt::Debug;

use num_traits::Zero;

use crate::traits::check::Check;
use crate::transform_as_validate_as_check;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct NonZero;

impl<V: Zero> Check<V> for NonZero {
    fn check(value: &V) -> bool {
        !value.is_zero()
    }
}

transform_as_validate_as_check!(impl[V] of [V] for NonZero where [V: Zero]);
