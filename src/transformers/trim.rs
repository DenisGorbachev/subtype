use crate::traits::transform::Transform;
use crate::traits::trim::Trim;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Trimmed;

impl<V> Transform<V> for Trimmed
where
    V: Trim,
{
    type Error = ();

    fn transform(value: V) -> Result<V, Self::Error> {
        Ok(value.trim())
    }
}
