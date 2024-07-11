use subtype::constraints::non_empty::NonEmpty;
use subtype::traits::transform::Transform;
use subtype::validation_error::ValidationError;
use subtype::{impl_constructor_setter, impl_try_from_ref_clone};

#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct Username(String);

impl_constructor_setter!(impl for Username, NonEmpty, String);
impl_try_from_ref_clone!( impl < 'a > TryFrom < & 'a String > for Username , < NonEmpty as  Transform < String >> :: Error , new );

fn main() {
    assert_eq!(Username::new(""), Err(ValidationError::<NonEmpty>::new()));
    assert_eq!(Username::new("alice"), Ok(Username("alice".to_string())));
}
