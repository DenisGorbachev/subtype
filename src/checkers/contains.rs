use std::marker::PhantomData;

use crate::transform_as_validate_as_check;
use crate::Check;
use crate::Conjure;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Contains<Needle>(PhantomData<Needle>);

impl<'a, Needle: Conjure<&'a str>> Check<String> for Contains<Needle> {
    fn check(value: &String) -> bool {
        value.contains(Needle::conjure())
    }
}

transform_as_validate_as_check!(impl['a, Prefix: Conjure<&'a str>] of [String] for Contains<Prefix>);

impl<'a, T: 'a + PartialEq, Needle: Conjure<&'a T>> Check<Vec<T>> for Contains<Needle> {
    fn check(value: &Vec<T>) -> bool {
        value.contains(Needle::conjure())
    }
}

transform_as_validate_as_check!(impl['a, T: 'a + PartialEq, Needle: Conjure<&'a T>] of [Vec<T>] for Contains<Needle>);
