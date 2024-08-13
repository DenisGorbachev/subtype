use subtype::checkers::not::Not;
use subtype::checkers::Empty;
use subtype::traits::validate::Validate;
use subtype::{impl_self_constructor_setter_with_validation, impl_try_from_own, impl_try_from_ref};
use subtype::{IncorrectValueError, ValidationError};

#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct Username(String);

type TheError = IncorrectValueError<String, <Not<Empty> as Validate<String>>::Error>;

impl_self_constructor_setter_with_validation!(impl for Username, String | Not<Empty>, tuple, value, new, set);
impl_try_from_own!(impl TryFrom<String> for Username, TheError, new);
impl_try_from_ref!(impl TryFrom<&String> for Username, TheError, new);

fn main() {
    assert_eq!(Username::new(""), Err(TheError::new("", ValidationError::new())));
    assert_eq!(Username::new("alice"), Ok(Username("alice".to_string())));
}
