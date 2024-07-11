use subtype::constraints::non_empty::NonEmpty;
use subtype::impl_all_with_validation;
use subtype::validation_error::ValidationError;

#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct Username(String);

impl_all_with_validation!(impl for Username, NonEmpty, String, tuple, value);

fn main() {
    assert_eq!(Username::new(""), Err(ValidationError::<NonEmpty>::new()));
    assert_eq!(Username::new("alice"), Ok(Username("alice".to_string())));
}
