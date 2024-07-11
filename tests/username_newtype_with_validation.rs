use subtype::constraints::non_empty::NonEmpty;
use subtype::validation_error::ValidationError;
use subtype::{impl_all, newtype_with_validation};

newtype_with_validation!(
    #[derive(PartialOrd, PartialEq, Clone, Debug)]
    pub struct Username(String | NonEmpty);
);

fn main() {
    assert_eq!(Username::new(""), Err(ValidationError::<NonEmpty>::new()));
    assert_eq!(Username::new("alice"), Ok(Username("alice".to_string())));
}
