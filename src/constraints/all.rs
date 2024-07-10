use crate::traits::check::Check;
use crate::traits::transform::Transform;
use crate::traits::validate::Validate;
use crate::validation_error::ValidationError;
use std::marker::PhantomData;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct All<Checker>(PhantomData<Checker>);

impl<Item, Value, Checker> Check<Value> for All<Checker>
where
    for<'a> &'a Value: IntoIterator<Item = &'a Item>,
    Checker: Check<Item>,
{
    fn check(value: &Value) -> bool {
        value.into_iter().all(|item| Checker::check(item))
    }
}

impl<Item, Value, Checker> Validate<Value> for All<Checker>
where
    for<'a> &'a Value: IntoIterator<Item = &'a Item>,
    Checker: Check<Item>,
{
    type Error = ValidationError<All<Checker>>;

    fn validate(value: &Value) -> Option<ValidationError<All<Checker>>> {
        if <All<Checker> as Check<Value>>::check(value) {
            None
        } else {
            Some(ValidationError::new())
        }
    }
}

impl<Item, Value, Checker> Transform<Value> for All<Checker>
where
    for<'a> &'a Value: IntoIterator<Item = &'a Item>,
    Checker: Check<Item>,
{
    type Error = ValidationError<All<Checker>>;

    fn transform(value: Value) -> Result<Value, ValidationError<All<Checker>>> {
        match <All<Checker> as Validate<Value>>::validate(&value) {
            None => Ok(value),
            Some(error) => Err(error),
        }
    }
}

// impl_validate_as_check!(All, <Validator>);
//
// impl_transform_as_validate!(All, <Checker>, where Checker: Check<>);
