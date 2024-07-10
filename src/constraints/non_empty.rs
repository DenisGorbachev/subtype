use crate::traits::check::Check;

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

// impl_validate_as_check!(NonEmpty);
//
// impl_transform_as_validate!(NonEmpty);
