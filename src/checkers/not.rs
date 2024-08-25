use std::marker::PhantomData;

use crate::Check;
use crate::TryTransform;
use crate::Validate;
use crate::{InvalidValueError, ValidationError};

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Not<T>(PhantomData<T>);

impl<Checker, Value> Check<Value> for Not<Checker>
where
    Checker: Check<Value>,
{
    fn check(value: &Value) -> bool {
        !Checker::check(value)
    }
}

impl<Validator, Value> Validate<Value> for Not<Validator>
where
    Validator: Validate<Value>,
{
    type Error = ValidationError<Not<Validator>>;

    fn validate(value: &Value) -> Option<Self::Error> {
        match Validator::validate(value) {
            None => Some(Self::Error::new()),
            Some(_) => None,
        }
    }
}

impl<Transformer, Value> TryTransform<Value> for Not<Transformer>
where
    Transformer: TryTransform<Value, Error = InvalidValueError<Value, Transformer>>,
    Value: Clone,
{
    type Error = InvalidValueError<Value, Not<Transformer>>;

    // TODO: Don't clone the value
    fn try_transform(value: Value) -> Result<Value, Self::Error> {
        match Transformer::try_transform(value.clone()) {
            Ok(_) => Err(Self::Error::new(value)),
            Err(_) => Ok(value),
        }
    }
}
