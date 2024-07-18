/// This error type is used for tuples of checkers (which can return any error)
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum ValidationError2<A, B> {
    Variant1(A),
    Variant2(B),
}
