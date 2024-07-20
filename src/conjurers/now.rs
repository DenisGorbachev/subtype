#[cfg(feature = "time")]
use time::OffsetDateTime;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Now;

#[cfg(feature = "time")]
impl crate::traits::conjure::Conjure<OffsetDateTime> for Now {
    fn conjure() -> OffsetDateTime {
        OffsetDateTime::now_utc()
    }
}
