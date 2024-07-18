use subtype::constraints::not::Not;
use subtype::constraints::Empty;
use subtype::constructor_from_transformer_checker;
use subtype::errors::InvalidValueError;
use subtype::transformers::trim::Trim;

#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct Username(String);

impl Username {
    constructor_from_transformer_checker!(pub fn new, String [Trim] | Not<Empty>, tuple, value);
}

#[test]
fn username_from_transformer_checker() {
    assert_eq!(Username::new(""), Err(InvalidValueError::<String, Not<Empty>>::new("")));
    assert_eq!(Username::new("   "), Err(InvalidValueError::<String, Not<Empty>>::new("")));
    assert_eq!(Username::new("alice"), Ok(Username("alice".to_string())));
}
