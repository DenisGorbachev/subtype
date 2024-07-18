use std::fmt::Debug;
use std::marker::PhantomData;

use crate::traits::check::Check;
use crate::traits::conjure::Conjure;
use crate::transform_as_validate_as_check;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Equal<Target>(PhantomData<Target>);

impl<Value, Target> Check<Value> for Equal<Target>
where
    Value: PartialEq,
    Target: Conjure<Value>,
{
    fn check(value: &Value) -> bool {
        value.eq(&Target::conjure())
    }
}

transform_as_validate_as_check!(impl[Value, Target] of [Value] for Equal<Target> where [
    Value: Eq,
    Target: Conjure<Value>,
]);
