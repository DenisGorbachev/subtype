pub trait Check<Value> {
    fn check(value: &Value) -> bool;
}
