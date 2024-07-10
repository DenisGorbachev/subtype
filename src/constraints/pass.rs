use crate::traits::check::Check;
use crate::traits::transform::Transform;
use crate::{impl_transform_as_validate, impl_validate_as_check};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Pass;

impl<T> Check<T> for Pass {
    fn check(_value: &T) -> bool {
        true
    }
}

impl_validate_as_check!(Pass);

impl_transform_as_validate!(Pass);
