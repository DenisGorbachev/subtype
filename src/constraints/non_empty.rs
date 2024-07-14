use crate::traits::check::Check;
use crate::transform_as_validate_as_check;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct NonEmpty;

impl Check<String> for NonEmpty {
    fn check(value: &String) -> bool {
        !value.is_empty()
    }
}

transform_as_validate_as_check!(impl of [String] for NonEmpty);

impl<T> Check<Vec<T>> for NonEmpty {
    fn check(value: &Vec<T>) -> bool {
        !value.is_empty()
    }
}

transform_as_validate_as_check!(impl[T] of [Vec<T>] for NonEmpty);
