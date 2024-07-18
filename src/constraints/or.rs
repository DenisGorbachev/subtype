use std::marker::PhantomData;

use crate::errors::{InvalidValueError, ValidationError};
use crate::traits::check::Check;
use crate::traits::transform::Transform;
use crate::traits::validate::Validate;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Or<A, B> {
    A(PhantomData<A>),
    B(PhantomData<B>),
}

impl<A, B, Value> Check<Value> for Or<A, B>
where
    A: Check<Value>,
    B: Check<Value>,
{
    fn check(value: &Value) -> bool {
        A::check(value) || B::check(value)
    }
}

impl<A, B, Value> Validate<Value> for Or<A, B>
where
    A: Validate<Value>,
    B: Validate<Value>,
{
    type Error = ValidationError<Or<A, B>>;

    fn validate(value: &Value) -> Option<Self::Error> {
        A::validate(value)
            .and_then(|_| B::validate(value))
            .map(|_| Self::Error::new())
    }
}

/// `Value: Clone` is required because `A::transform` takes ownership of the value
impl<A, B, Value> Transform<Value> for Or<A, B>
where
    A: Transform<Value>,
    B: Transform<Value>,
    Value: Clone,
{
    type Error = InvalidValueError<Value, Or<A, B>>;

    fn transform(value: Value) -> Result<Value, Self::Error> {
        A::transform(value.clone())
            .or_else(|_| B::transform(value.clone()))
            .map_err(|_| Self::Error::new(value))
    }
}
