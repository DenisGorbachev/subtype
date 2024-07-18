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
    pub struct UsernameRegularNotEmpty {
        inner: String | Not<Empty>
    }
);

newtype!(
    #[derive(PartialOrd, PartialEq, Clone, Debug)]
    pub struct UsernameTupleAny(String);
);

newtype!(
    #[derive(PartialOrd, PartialEq, Clone, Debug)]
    pub struct UsernameRegularAny {
        inner: String,
    }
);

#[test]
fn username_newtype_with_validation() {
    type TheError = IncorrectValueError<String, <Not<Empty> as Validate<String>>::Error>;
    assert_eq!(UsernameTupleNotEmpty::new(""), Err(TheError::new("", ValidationError::new())));
    assert_eq!(UsernameTupleNotEmpty::new("alice"), Ok(UsernameTupleNotEmpty("alice".to_string())));
    assert_eq!(UsernameRegularNotEmpty::new(""), Err(TheError::new("", ValidationError::new())));
    assert_eq!(
        UsernameRegularNotEmpty::new("alice"),
        Ok(UsernameRegularNotEmpty {
            inner: "alice".to_string()
        })
    );
    assert_eq!(UsernameTupleAny::new(""), UsernameTupleAny("".to_string()));
    assert_eq!(UsernameTupleAny::new("alice"), UsernameTupleAny("alice".to_string()));
    assert_eq!(
        UsernameRegularAny::new(""),
        UsernameRegularAny {
            inner: "".to_string()
        }
    );
    assert_eq!(
        UsernameRegularAny::new("alice"),
        UsernameRegularAny {
            inner: "alice".to_string()
        }
    );
}
