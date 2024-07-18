use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;

use derive_more::Error;
use pretty_type_name::pretty_type_name;

#[derive(Error, Eq, PartialEq, Hash, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InvalidValueError<Value, Validator> {
    value: Value,
    validator: PhantomData<Validator>,
}

impl<Value, Validator> InvalidValueError<Value, Validator> {
    pub fn new(value: impl Into<Value>) -> Self {
        Self {
            value: value.into(),
            validator: PhantomData,
        }
    }
}

impl<Value, Validator> From<Value> for InvalidValueError<Value, Validator> {
    fn from(value: Value) -> Self {
        Self::new(value)
    }
}

// impl<Value: Debug, Validator: Debug> Display for InvalidValueError<Value, Validator> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         <Self as Debug>::fmt(self, f)
//     }
// }

impl<Value: Debug, Validator> Display for InvalidValueError<Value, Validator> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> ::core::fmt::Result {
        f.debug_struct("InvalidValueError")
            .field("value", &self.value)
            .field("validator", &pretty_type_name::<Validator>())
            .finish()
    }
}
