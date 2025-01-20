use std::marker::PhantomData;

use derive_more::Error;

use crate::Check;
use crate::TryTransform;
use crate::Validate;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct AllChars<Checker>(PhantomData<Checker>);

// TODO: Use fmt_derive
#[derive(Error, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct AllCharsError<Error> {
    pub invalid_index: usize,
    pub error: Error,
}

impl<Error> AllCharsError<Error> {
    pub fn new(invalid_index: usize, error: Error) -> Self {
        Self {
            invalid_index,
            error,
        }
    }
}

impl<Checker> Check<String> for AllChars<Checker>
where
    Checker: Check<char>,
{
    fn check(value: &String) -> bool {
        value.chars().all(|item| Checker::check(&item))
    }
}

impl<Validator> Validate<String> for AllChars<Validator>
where
    Validator: Validate<char>,
{
    type Error = AllCharsError<<Validator as Validate<char>>::Error>;

    fn validate(value: &String) -> Option<Self::Error> {
        value
            .char_indices()
            .find_map(|(index, item)| Validator::validate(&item).map(|error| AllCharsError::new(index, error)))
    }
}

impl<Transformer> TryTransform<String> for AllChars<Transformer>
where
    Transformer: TryTransform<char>,
{
    type Error = AllCharsError<<Transformer as TryTransform<char>>::Error>;

    fn try_transform(value: String) -> Result<String, Self::Error> {
        value
            .char_indices()
            .map(|(index, item)| Transformer::try_transform(item).map_err(|error| AllCharsError::new(index, error)))
            .collect()
    }
}
