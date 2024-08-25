use crate::Conjure;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Space;

impl Conjure<char> for Space {
    fn conjure() -> char {
        ' '
    }
}

impl Conjure<&str> for Space {
    fn conjure() -> &'static str {
        " "
    }
}
