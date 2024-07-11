use subtype::constraints::non_empty::NonEmpty;
use subtype::traits::transform::Transform;
use subtype::validation_error::ValidationError;
use subtype::{constructor, impl_try_from_owned_as_delegate, setter};

#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct Email(String);

impl Email {
    constructor!(pub fn new, Email, NonEmpty, String);
    setter!(pub fn set, Email, NonEmpty, String);
}
impl_try_from_owned_as_delegate!(impl TryFrom<String> for Email, new, <NonEmpty as Transform<String>>::Error);

fn main() {
    assert_eq!(Email::new(""), Err(ValidationError::<NonEmpty>::new()))
}
