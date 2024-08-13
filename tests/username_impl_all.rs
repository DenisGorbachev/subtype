use subtype::checkers::not::Not;
use subtype::checkers::Empty;
use subtype::impl_all_with_validation;
use subtype::traits::validate::Validate;
use subtype::{IncorrectValueError, ValidationError};

#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct Username(String);

impl_all_with_validation!(impl for Username, String | Not<Empty>, tuple, value);

#[test]
fn username_impl_all() {
    type TheError = IncorrectValueError<String, <Not<Empty> as Validate<String>>::Error>;
    assert_eq!(Username::new(""), Err(TheError::new("", ValidationError::new())));
    assert_eq!(Username::new("alice"), Ok(Username("alice".to_string())));
}
