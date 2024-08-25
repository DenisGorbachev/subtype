use subtype::transformers::trim::Trim;
use subtype::Empty;
use subtype::Not;
use subtype::Validate;
use subtype::{constructor_with_validation, impl_try_from_own, impl_try_from_ref, setter_with_validation};
use subtype::{IncorrectValueError, ValidationError};

#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct Username(String);

impl Username {
    constructor_with_validation!(pub fn new, String [Trim] | Not<Empty>, tuple, value);
    setter_with_validation!(pub fn set, String [Trim] | Not<Empty>, tuple, value);
}

type TheError = IncorrectValueError<String, <Not<Empty> as Validate<String>>::Error>;

impl_try_from_own!(impl TryFrom<String> for Username, TheError, new);
impl_try_from_ref!(impl TryFrom<&String> for Username, TheError, new, Clone::clone);
impl_try_from_ref!(impl TryFrom<&str> for Username, TheError, new);

#[test]
fn username_explicit() {
    assert_eq!(Username::new(""), Err(TheError::new("", ValidationError::new())));
    assert_eq!(Username::new("alice"), Ok(Username("alice".to_string())));
}
