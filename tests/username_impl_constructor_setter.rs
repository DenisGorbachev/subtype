use subtype::IsEmpty;
use subtype::Not;
use subtype::Validate;
use subtype::{impl_self_constructor_setter_with_generic_validation, impl_try_from_own, impl_try_from_ref};
use subtype::{IncorrectValueError, ValidationError};

#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct Username(String);

type TheError = IncorrectValueError<String, <Not<IsEmpty> as Validate<String>>::Error>;

impl_self_constructor_setter_with_generic_validation!(impl for Username, String | Not<IsEmpty>, tuple, value, new, set);
impl_try_from_own!(impl TryFrom<String> for Username, TheError, new);
impl_try_from_ref!(impl TryFrom<&String> for Username, TheError, new);

#[test]
fn username_impl_constructor_setter() {
    assert_eq!(Username::new(""), Err(TheError::new("", ValidationError::new())));
    assert_eq!(Username::new("alice"), Ok(Username("alice".to_string())));
}
