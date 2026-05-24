use subtype::Inclusive;
use subtype::IsEmpty;
use subtype::MaxLen;
use subtype::Not;
use subtype::Trim;
use subtype::{IncorrectValueError, ValidationError, ValidationError2};
use subtype::{constructor_with_generic_validation, purify_with_generic_validation};

#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct Username(String);

type MaxLen255 = MaxLen<255, Inclusive>;

impl Username {
    constructor_with_generic_validation!(pub fn new, String [Trim] | (Not<IsEmpty>, MaxLen255) [Trim], tuple, value);
    purify_with_generic_validation!(pub fn purify, String [Trim] | (Not<IsEmpty>, MaxLen255) [Trim], tuple, value);
}

// #[derive(Error, Display, From, Eq, PartialEq, Hash, Clone, Debug)]
// pub enum UsernameError {
//     NotEmpty(IncorrectValueError<String, Not<Empty>>),
//     MaxLen(IncorrectValueError<String, MaxLen<255, Inclusive>>),
// }

#[test]
fn username_explicit() {
    type TheTupleError = ValidationError2<ValidationError<Not<IsEmpty>>, ValidationError<MaxLen255>>;
    type TheError = IncorrectValueError<String, TheTupleError>;
    assert_eq!(Username::new(""), Err(TheError::new("", TheTupleError::Variant1(ValidationError::new()))));
    assert_eq!(Username::new("alice"), Ok(Username("alice".to_string())));
    assert_eq!(Username::new("a".repeat(THIS_IS_SPARTA)), Err(TheError::new("a".repeat(THIS_IS_SPARTA), TheTupleError::Variant2(ValidationError::new()))));
}

const THIS_IS_SPARTA: usize = 300;
