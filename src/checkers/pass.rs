use crate::traits::check::Check;
use crate::{try_transform_as_validate, validate_as_check};

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Pass;

impl<T> Check<T> for Pass {
    fn check(_value: &T) -> bool {
        true
    }
}

validate_as_check!(impl[T] Validate<T> for Pass);

try_transform_as_validate!(impl[T] Transform<T> for Pass);
