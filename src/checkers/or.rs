use std::marker::PhantomData;

use crate::errors::{InvalidValueError, ValidationError};
use crate::Check;
use crate::TryTransform;
use crate::Validate;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
#[allow(dead_code)]
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
impl<A, B, Value> TryTransform<Value> for Or<A, B>
where
    A: TryTransform<Value>,
    B: TryTransform<Value>,
    Value: Clone,
{
    type Error = InvalidValueError<Value, Or<A, B>>;

    fn try_transform(value: Value) -> Result<Value, Self::Error> {
        A::try_transform(value.clone())
            .or_else(|_| B::try_transform(value.clone()))
            .map_err(|_| Self::Error::new(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conjurers::i32::I32;
    use crate::{GreaterThan, LessThan};

    #[test]
    fn must_return_correct_values() {
        type Or1 = Or<LessThan<I32<10>>, GreaterThan<I32<50>>>;
        assert!(Or1::check(&0));
        assert!(!Or1::check(&15));
        assert!(Or1::check(&55));
    }
}
