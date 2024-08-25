use std::marker::PhantomData;

use derive_more::Error;

use crate::Check;
use crate::TryTransform;
use crate::Validate;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct All<Checker>(PhantomData<Checker>);

// TODO: Use fmt_derive
#[derive(Error, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct AllError<Error> {
    pub invalid_index: usize,
    pub error: Error,
}

impl<Error> AllError<Error> {
    pub fn new(invalid_index: usize, error: Error) -> Self {
        Self {
            invalid_index,
            error,
        }
    }
}

impl<Item, Value, Checker> Check<Value> for All<Checker>
where
    for<'a> &'a Value: IntoIterator<Item = &'a Item>,
    Checker: Check<Item>,
{
    fn check(value: &Value) -> bool {
        value.into_iter().all(|item| Checker::check(item))
    }
}

impl<Item, Value, Validator> Validate<Value> for All<Validator>
where
    for<'a> &'a Value: IntoIterator<Item = &'a Item>,
    Validator: Validate<Item>,
{
    type Error = AllError<<Validator as Validate<Item>>::Error>;

    fn validate(value: &Value) -> Option<Self::Error> {
        value
            .into_iter()
            .enumerate()
            .find_map(|(index, item)| Validator::validate(item).map(|error| AllError::new(index, error)))
    }
}

impl<Item, Value, Transformer> TryTransform<Value> for All<Transformer>
where
    Value: IntoIterator<Item = Item> + FromIterator<Item>,
    Transformer: TryTransform<Item>,
{
    type Error = AllError<<Transformer as TryTransform<Item>>::Error>;

    fn try_transform(value: Value) -> Result<Value, Self::Error> {
        value
            .into_iter()
            .enumerate()
            .map(|(index, item)| Transformer::try_transform(item).map_err(|error| AllError::new(index, error)))
            .collect()
    }
}
