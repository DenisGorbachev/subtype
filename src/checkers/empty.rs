use crate::transform_as_validate_as_check;
use crate::Check;
use crate::IsEmpty;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Empty;

impl<T: IsEmpty> Check<T> for Empty {
    fn check(value: &T) -> bool {
        value.is_empty()
    }
}

transform_as_validate_as_check!(impl[T: IsEmpty] of [T] for Empty);
