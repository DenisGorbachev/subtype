/// `Transform` is a trait for infallible transformations.
///
/// For fallible transformations, see [`TryTransform`](crate::TryTransform)
pub trait Transform<Value> {
    fn transform(value: Value) -> Value;
}
