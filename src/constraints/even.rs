use std::fmt::Debug;

use crate::traits::check::Check;
use crate::traits::even::IsEven;
use crate::transform_as_validate_as_check;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Even;

impl<V: IsEven> Check<V> for Even {
    fn check(value: &V) -> bool {
        !value.is_even()
    }
}

transform_as_validate_as_check!(impl[V] of [V] for Even where [V: IsEven]);
