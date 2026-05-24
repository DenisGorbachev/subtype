use crate::Check;
use crate::transform_as_validate_as_check;
use standard_traits::IsUppercase as IsUppercaseTrait;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct IsUppercase;

impl<T: IsUppercaseTrait> Check<T> for IsUppercase {
    fn check(value: &T) -> bool {
        value.is_uppercase()
    }
}

transform_as_validate_as_check!(impl[T: IsUppercaseTrait] of [T] for IsUppercase);
