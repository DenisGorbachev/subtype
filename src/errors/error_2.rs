/// This error type is used for tuples of checkers (which can return any error)
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Error2<A, B> {
    A(A),
    B(B),
}
