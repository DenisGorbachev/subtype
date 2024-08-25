use std::marker::PhantomData;

use crate::validate_as_check;
use crate::Check;
use crate::Conjure;
use standard_traits::Len;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct MaxLen<const MAXIMUM: usize, Inclusivity>(PhantomData<Inclusivity>);

impl<Value, const MAXIMUM: usize, Inclusivity> Check<Value> for MaxLen<MAXIMUM, Inclusivity>
where
    Value: Len,
    Inclusivity: Conjure<bool>,
{
    fn check(value: &Value) -> bool {
        if Inclusivity::conjure() {
            value.len() <= MAXIMUM
        } else {
            value.len() < MAXIMUM
        }
    }
}

validate_as_check!(impl[Value, const MAXIMUM: usize, Inclusivity] Validate<Value> for MaxLen<MAXIMUM, Inclusivity> where [
    Value: Len,
    Inclusivity: Conjure<bool>
]);
