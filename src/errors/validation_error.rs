use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

use derive_more::Error;
use pretty_type_name::pretty_type_name;

#[derive(Error, Eq, PartialEq, Hash, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValidationError<Validator>(PhantomData<Validator>);

impl<Validator> ValidationError<Validator> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Validator> Default for ValidationError<Validator> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Validator> Display for ValidationError<Validator> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValidationError")
            .field("validator", &pretty_type_name::<Validator>())
            .finish()
    }
}
