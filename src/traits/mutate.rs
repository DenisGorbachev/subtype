pub trait Mutate<Value> {
    fn mutate(value: &mut Value);
}
