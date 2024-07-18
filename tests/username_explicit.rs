use subtype::constraints::not::Not;
use subtype::constraints::Empty;
use subtype::errors::InvalidValueError;
use subtype::traits::try_transform::TryTransform;
use subtype::{constructor_with_validation, impl_try_from_own, impl_try_from_ref, setter_with_validation};

#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct Username(String);

impl Username {
    constructor_with_validation!(pub fn new, Not<Empty>, String, tuple, value);
    setter_with_validation!(pub fn set, Not<Empty>, String, tuple, value);
}
impl_try_from_own!(impl TryFrom<String> for Username, <Not<Empty> as TryTransform<String>>::Error, new);
impl_try_from_ref!(impl TryFrom<&String> for Username, <Not<Empty> as TryTransform<String>>::Error, new, Clone::clone);
impl_try_from_ref!(impl TryFrom<&str> for Username, <Not<Empty> as TryTransform<String>>::Error, new);

#[test]
fn username_explicit() {
    assert_eq!(Username::new(""), Err(InvalidValueError::<String, Not<Empty>>::new("")));
    assert_eq!(Username::new("alice"), Ok(Username("alice".to_string())));
}
