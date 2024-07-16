use subtype::constraints::non_empty::NonEmpty;
use subtype::errors::InvalidValueError;
use subtype::impl_all_with_validation;

#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct Username(String);

impl_all_with_validation!(impl for Username, NonEmpty, String, tuple, value);

#[test]
fn username_impl_all() {
    assert_eq!(Username::new(""), Err(InvalidValueError::<String, NonEmpty>::new("")));
    assert_eq!(Username::new("alice"), Ok(Username("alice".to_string())));
}
