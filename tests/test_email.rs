use subtype::constraints::non_empty::NonEmpty;
use subtype::impl_new;
use subtype::validation_error::ValidationError;

#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct Email(String);

impl_new!(NonEmpty, String, impl for Email);

fn main() {
    assert_eq!(Email::new(""), Err(ValidationError::<NonEmpty>::new()))
}
