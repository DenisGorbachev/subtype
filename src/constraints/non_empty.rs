use crate::traits::check::Check;
use crate::{transform_as_validate, validate_as_check};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct NonEmpty;

impl Check<String> for NonEmpty {
    fn check(value: &String) -> bool {
        !value.is_empty()
    }
}

impl<T> Check<Vec<T>> for NonEmpty {
    fn check(value: &Vec<T>) -> bool {
        !value.is_empty()
    }
}

validate_as_check!(impl Validate<String> for NonEmpty);

validate_as_check!(impl<T> Validate<Vec<T>> for NonEmpty);

transform_as_validate!(impl Transform<String> for NonEmpty);

transform_as_validate!(impl<T> Transform<Vec<T>> for NonEmpty);
