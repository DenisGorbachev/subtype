use subtype::IsEmpty;
use subtype::Not;
use subtype::Trim;
use subtype::Validate;
use subtype::{constructor_with_generic_validation, impl_try_from_own, impl_try_from_ref, setter_with_generic_validation};
use subtype::{IncorrectValueError, ValidationError};

#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct Username(String);

impl Username {
    constructor_with_generic_validation!(pub fn new, String [Trim] | Not<IsEmpty>, tuple, value);
    setter_with_generic_validation!(pub fn set, String [Trim] | Not<IsEmpty>, tuple, value);
}

type TheError = IncorrectValueError<String, <Not<IsEmpty> as Validate<String>>::Error>;

impl_try_from_own!(impl TryFrom<String> for Username, TheError, new);
impl_try_from_ref!(impl TryFrom<&String> for Username, TheError, new, Clone::clone);
impl_try_from_ref!(impl TryFrom<&str> for Username, TheError, new);

#[test]
fn username_explicit() {
    assert_eq!(Username::new(""), Err(TheError::new("", ValidationError::new())));
    assert_eq!(Username::new("alice"), Ok(Username("alice".to_string())));
}
