use std::marker::PhantomData;

use crate::traits::check::Check;
use crate::traits::conjure::Conjure;
use crate::transform_as_validate_as_check;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct StartsWith<Prefix>(PhantomData<Prefix>);

impl<'a, Prefix: Conjure<&'a str>> Check<String> for StartsWith<Prefix> {
    fn check(value: &String) -> bool {
        value.starts_with(Prefix::conjure())
    }
}

transform_as_validate_as_check!(impl['a, Prefix: Conjure<&'a str>] of [String] for StartsWith<Prefix>);

impl<'a, T: 'a + PartialEq, Prefix: Conjure<&'a [T]>> Check<Vec<T>> for StartsWith<Prefix> {
    fn check(value: &Vec<T>) -> bool {
        value.starts_with(Prefix::conjure())
    }
}

transform_as_validate_as_check!(impl['a, T: 'a + PartialEq, Prefix: Conjure<&'a [T]>] of [Vec<T>] for StartsWith<Prefix>);
