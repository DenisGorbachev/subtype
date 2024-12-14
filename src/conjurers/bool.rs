use crate::Conjure;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Bool<const VALUE: bool>;

impl<const VALUE: bool> From<Bool<VALUE>> for bool {
    fn from(_v: Bool<VALUE>) -> Self {
        VALUE
    }
}

impl<const VALUE: bool> Conjure<bool> for Bool<VALUE> {
    fn conjure() -> bool {
        VALUE
    }
}
