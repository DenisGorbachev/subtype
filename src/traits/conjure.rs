/// This trait allows to convert a type to a value. This is different from the `From` trait because it works without a specific value (doesn't take a `self` parameter).
///
/// Compare:
///
/// * `let a: A = b.into()` using `Into<A> for B` to get `a: A` from `b` (value).
/// * `let a: A = B::conjure()` using `Conjure<A> for B` to get `a: A` from `B` (type).
///
/// This has better performance than `B::default().into()` because it avoids creating an intermediate value.
pub trait Conjure<T> {
    fn conjure() -> T;
}
