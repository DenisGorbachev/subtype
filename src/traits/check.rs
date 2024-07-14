pub trait Check<Value> {
    fn check(value: &Value) -> bool;
}

impl<A, B, Value> Check<Value> for (A, B)
where
    A: Check<Value>,
    B: Check<Value>,
{
    fn check(value: &Value) -> bool {
        A::check(value) && B::check(value)
    }
}
