use crate::traits::conjure::Conjure;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Exclusive;

impl Conjure<bool> for Exclusive {
    fn conjure() -> bool {
        false
    }
}
