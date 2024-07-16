use subtype::constraints::non_empty::NonEmpty;
use subtype::errors::InvalidValueError;
use subtype::traits::transform::Transform;
use subtype::{impl_self_constructor_setter_with_validation, impl_try_from_own, impl_try_from_ref};

pub mod macro_name_scope;

#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct Username(String);

impl_self_constructor_setter_with_validation!(impl for Username, NonEmpty, String, tuple, value, new, set);
impl_try_from_own!(impl TryFrom<String> for Username, <NonEmpty as Transform<String>>::Error, new);
impl_try_from_ref!(impl TryFrom<&String> for Username, <NonEmpty as Transform<String>>::Error, new);

fn main() {
    assert_eq!(Username::new(""), Err(InvalidValueError::<String, NonEmpty>::new("")));
    assert_eq!(Username::new("alice"), Ok(Username("alice".to_string())));
}
