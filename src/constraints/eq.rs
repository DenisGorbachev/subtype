use std::fmt::Debug;
use std::marker::PhantomData;

use crate::traits::check::Check;
use crate::traits::conjure::Conjure;
use crate::transform_as_validate_as_check;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Equal<ValueConjurer>(PhantomData<ValueConjurer>);

impl<ValueConjurer, Value> Check<Value> for Equal<ValueConjurer>
where
    ValueConjurer: Conjure<Value>,
    Value: Eq,
{
    fn check(value: &Value) -> bool {
        value.eq(&ValueConjurer::conjure())
    }
}

transform_as_validate_as_check!(impl[ValueConjurer, Value] of [Value] for Equal<ValueConjurer> where [
    ValueConjurer: Conjure<Value>,
    Value: Eq,
]);
