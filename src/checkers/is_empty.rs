use crate::transform_as_validate_as_check;
use crate::Check;
use standard_traits::IsEmpty as IsEmptyTrait;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct IsEmpty;

impl<T: IsEmptyTrait> Check<T> for IsEmpty {
    fn check(value: &T) -> bool {
        value.is_empty()
    }
}

transform_as_validate_as_check!(impl[T: IsEmptyTrait] of [T] for IsEmpty);
