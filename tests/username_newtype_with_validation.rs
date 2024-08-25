use subtype::newtype;
use subtype::transformers::trim::Trim;
use subtype::Empty;
use subtype::Not;
use subtype::Validate;
use subtype::{IncorrectValueError, ValidationError};

newtype!(
    #[derive(PartialOrd, PartialEq, Clone, Debug)]
    pub struct UsernameNotEmpty(String [Trim] | Not<Empty>);
);

newtype!(
    #[derive(PartialOrd, PartialEq, Clone, Debug)]
    pub struct UsernameTrim(String [Trim]);
);

newtype!(
    #[derive(PartialOrd, PartialEq, Clone, Debug)]
    pub struct UsernamePlain(String);
);

#[test]
fn username_newtype_with_validation() {
    type TheError = IncorrectValueError<String, <Not<Empty> as Validate<String>>::Error>;
    assert_eq!(UsernameNotEmpty::new(""), Err(TheError::new("", ValidationError::new())));
    assert_eq!(UsernameNotEmpty::new("alice"), Ok(UsernameNotEmpty("alice".to_string())));
    assert_eq!(UsernameTrim::new(" alice ").as_ref(), "alice");
    assert_eq!(UsernamePlain::new(""), UsernamePlain("".to_string()));
    assert_eq!(UsernamePlain::new("alice"), UsernamePlain("alice".to_string()));
}
