use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

use derive_more::Error;

#[derive(Error, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct ValidationError<Checker> {
    phantom: PhantomData<Checker>,
}

impl<Checker> Default for ValidationError<Checker> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Checker> ValidationError<Checker> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<Validator: Default + Display> Display for ValidationError<Validator> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "validation error: {}", Validator::default())
    }
}
