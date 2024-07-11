use subtype::constraints::non_empty::NonEmpty;
use subtype::traits::transform::Transform;
use subtype::validation_error::ValidationError;
use subtype::{constructor, impl_try_from_own, impl_try_from_ref_clone, setter};

#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct Username(String);

impl Username {
    constructor!(pub fn new, NonEmpty, String, tuple, value);
    setter!(pub fn set, NonEmpty, String, tuple, value);
}
impl_try_from_own!(impl TryFrom<String> for Username, <NonEmpty as Transform<String>>::Error, new);
impl_try_from_ref_clone!(impl<'a> TryFrom<&'a String> for Username, <NonEmpty as Transform<String>>::Error, new);
impl_try_from_ref_clone!(impl<'a> TryFrom<&'a str> for Username, <NonEmpty as Transform<String>>::Error, new);

fn main() {
    assert_eq!(Username::new(""), Err(ValidationError::<NonEmpty>::new()));
    assert_eq!(Username::new("alice"), Ok(Username("alice".to_string())));
}
