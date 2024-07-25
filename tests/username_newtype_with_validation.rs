use subtype::checkers::not::Not;
use subtype::checkers::Empty;
use subtype::errors::{IncorrectValueError, ValidationError};
use subtype::newtype;
use subtype::traits::validate::Validate;

newtype!(
    #[derive(PartialOrd, PartialEq, Clone, Debug)]
    pub struct UsernameTupleNotEmpty(String | Not<Empty>);
);

newtype!(
    #[derive(PartialOrd, PartialEq, Clone, Debug)]
    pub struct UsernameTupleAny(String);
);

#[test]
fn username_newtype_with_validation() {
    type TheError = IncorrectValueError<String, <Not<Empty> as Validate<String>>::Error>;
    assert_eq!(UsernameTupleNotEmpty::new(""), Err(TheError::new("", ValidationError::new())));
    assert_eq!(UsernameTupleNotEmpty::new("alice"), Ok(UsernameTupleNotEmpty("alice".to_string())));
    assert_eq!(UsernameTupleAny::new(""), UsernameTupleAny("".to_string()));
    assert_eq!(UsernameTupleAny::new("alice"), UsernameTupleAny("alice".to_string()));
}
