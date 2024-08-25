use crate::transform_as_validate_as_check;
use crate::Check;
use crate::Conjure;
use crate::GetRef;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct FieldEqual<Field, Target>(PhantomData<Field>, PhantomData<Target>);

impl<Value, Field, Target> Check<Value> for FieldEqual<Field, Target>
where
    Value: PartialEq,
    Field: GetRef<Value, Target>,
    Target: Conjure<Value>,
{
    fn check(value: &Value) -> bool {
        value.eq(&Target::conjure())
    }
}

transform_as_validate_as_check!(impl[Value, Field, Target] of [Value] for FieldEqual<Field, Target> where [
    Value: Eq,
    Field: GetRef<Value, Target>,
    Target: Conjure<Value>,
]);
