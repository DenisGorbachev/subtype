use std::fmt::Debug;

use num_traits::Zero as ZeroTrait;

use crate::transform_as_validate_as_check;
use crate::Check;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Zero;

impl<V: ZeroTrait> Check<V> for Zero {
    fn check(value: &V) -> bool {
        value.is_zero()
    }
}

transform_as_validate_as_check!(impl[V] of [V] for Zero where [V: ZeroTrait]);
