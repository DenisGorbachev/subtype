/// This trait allows to convert a type to a value. This is different from the `From` trait because it works without a specific value (doesn't take a `self` parameter).
///
/// Compare:
///
/// * `Into<A> for B` gets `a: A` from `b: B` via `let a: A = b.into()`.
/// * `Value<A> for B` gets `a: A` from `B` via `let a = B::value()`.
///
/// This has better performance than `B::default().into()` because it avoids creating an intermediate value.
pub trait Value<T> {
    fn value() -> T;
}
