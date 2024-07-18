use std::fmt::{Debug, Display, Formatter};

use derive_more::Error;

#[derive(Error, Eq, PartialEq, Hash, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IncorrectValueError<Value, Error> {
    pub value: Value,
    pub error: Error,
}

impl<Value, Error> IncorrectValueError<Value, Error> {
    pub fn new(value: impl Into<Value>, error: impl Into<Error>) -> Self {
        Self {
            value: value.into(),
            error: error.into(),
        }
    }
}

impl<Value: Debug, Error: Debug> Display for IncorrectValueError<Value, Error> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> ::core::fmt::Result {
        f.debug_struct("IncorrectValueError")
            .field("value", &self.value)
            .field("error", &self.error)
            .finish()
    }
}
