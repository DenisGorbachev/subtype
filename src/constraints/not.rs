use std::marker::PhantomData;

use crate::errors::ValidationError;
use crate::traits::check::Check;
use crate::traits::transform::Transform;
use crate::traits::validate::Validate;

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

impl<Transformer, Value> Transform<Value> for Not<Transformer>
where
    Transformer: Transform<Value>,
{
    type Error = ValidationError<Not<Transformer>>;

    fn transform(value: Value) -> Result<Value, Self::Error> {
        Ok(value)
    }
}
