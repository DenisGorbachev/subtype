use crate::traits::trim::Trim;
use crate::traits::try_transform::TryTransform;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Trimmed;

impl<V> TryTransform<V> for Trimmed
where
    V: Trim,
{
    type Error = ();

    fn try_transform(value: V) -> Result<V, Self::Error> {
        Ok(value.trim())
    }
}
