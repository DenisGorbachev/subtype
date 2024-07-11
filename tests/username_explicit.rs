use subtype::constraints::non_empty::NonEmpty;
use subtype::traits::transform::Transform;
use subtype::validation_error::ValidationError;
use subtype::{constructor_with_validation, impl_try_from_own, impl_try_from_ref, setter_with_validation};

#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct Username(String);

impl Username {
    constructor_with_validation!(pub fn new, NonEmpty, String, tuple, value);
    setter_with_validation!(pub fn set, NonEmpty, String, tuple, value);
}
impl_try_from_own!(impl TryFrom<String> for Username, <NonEmpty as Transform<String>>::Error, new);
impl_try_from_ref!(impl<'a> TryFrom<&'a String> for Username, <NonEmpty as Transform<String>>::Error, new, Clone::clone);
impl_try_from_ref!(impl<'a> TryFrom<&'a str> for Username, <NonEmpty as Transform<String>>::Error, new);

fn main() {
    assert_eq!(Username::new(""), Err(ValidationError::<NonEmpty>::new()));
    assert_eq!(Username::new("alice"), Ok(Username("alice".to_string())));
}
