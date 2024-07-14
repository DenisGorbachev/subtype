#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum BinaryError<A, B> {
    VariantA(A),
    VariantB(B),
}
