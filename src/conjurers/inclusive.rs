use crate::Conjure;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Inclusive;

impl Conjure<bool> for Inclusive {
    fn conjure() -> bool {
        true
    }
}
