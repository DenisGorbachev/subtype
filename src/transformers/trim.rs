use crate::traits::transform::Transform;
use crate::traits::trim::Trim as TrimTrait;
use crate::traits::try_transform::TryTransform;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Trim;

impl<V> Transform<V> for Trim
where
    V: TrimTrait,
{
    fn transform(value: V) -> V {
        value.trim()
    }
}

impl<V> TryTransform<V> for Trim
where
    V: TrimTrait,
{
    type Error = ();

    fn try_transform(value: V) -> Result<V, Self::Error> {
        Ok(value.trim())
    }
}
