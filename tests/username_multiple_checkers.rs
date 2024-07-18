use derive_more::{Display, Error, From};

use subtype::checkers::max_len::MaxLen;
use subtype::checkers::not::Not;
use subtype::checkers::Empty;
use subtype::constructor_with_validation;
use subtype::containers::inclusive::Inclusive;
use subtype::errors::InvalidValueError;
use subtype::transformers::trim::Trim;

#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct Username(String);

impl Username {
    constructor_with_validation!(pub fn new, String [Trim] | Not<Empty> | MaxLen<255, Inclusive> { UsernameError } [Trim] , tuple, value);
}

#[derive(Error, Display, From, Eq, PartialEq, Hash, Clone, Debug)]
pub enum UsernameError {
    NotEmpty(InvalidValueError<String, Not<Empty>>),
    MaxLen(InvalidValueError<String, MaxLen<255, Inclusive>>),
}

#[test]
fn username_explicit() {
    assert_eq!(Username::new(""), Err(UsernameError::NotEmpty(InvalidValueError::<String, Not<Empty>>::new(""))));
    assert_eq!(Username::new("alice"), Ok(Username("alice".to_string())));
    assert_eq!(Username::new("a".repeat(300)), Err(UsernameError::MaxLen(InvalidValueError::<String, MaxLen<255, Inclusive>>::new("a".repeat(300)))));
}
