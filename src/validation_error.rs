use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

use derive_more::Error;
use pretty_type_name::pretty_type_name;

#[derive(Error, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct ValidationError<Validator> {
    phantom: PhantomData<Validator>,
}

impl<Validator> ValidationError<Validator> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<Validator> Default for ValidationError<Validator> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Validator> Display for ValidationError<Validator> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "validation error: {}", pretty_type_name::<Validator>())
    }
}
