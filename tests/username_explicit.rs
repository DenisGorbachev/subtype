use subtype::checkers::not::Not;
use subtype::checkers::Empty;
use subtype::errors::InvalidValueError;
use subtype::transformers::trim::Trim;
use subtype::{constructor_with_validation, impl_try_from_own, impl_try_from_ref, setter_with_validation};

#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct Username(String);

impl Username {
    constructor_with_validation!(pub fn new, String [Trim] | Not<Empty>, tuple, value);
    setter_with_validation!(pub fn set, String [Trim] | Not<Empty>, tuple, value);
}
impl_try_from_own!(impl TryFrom<String> for Username, InvalidValueError<String, Not<Empty>>, new);
impl_try_from_ref!(impl TryFrom<&String> for Username, InvalidValueError<String, Not<Empty>>, new, Clone::clone);
impl_try_from_ref!(impl TryFrom<&str> for Username, InvalidValueError<String, Not<Empty>>, new);

#[test]
fn username_explicit() {
    assert_eq!(Username::new(""), Err(InvalidValueError::<String, Not<Empty>>::new("")));
    assert_eq!(Username::new("alice"), Ok(Username("alice".to_string())));
}
