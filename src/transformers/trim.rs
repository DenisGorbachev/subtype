use crate::traits::adjust::Adjust;
use crate::traits::transform::Transform;
use crate::traits::trim::Trim as TrimTrait;
use crate::traits::try_transform::TryTransform;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Trim;

impl<V> Transform<V> for Trim
where
    V: TrimTrait,
{
    fn transform(mut value: V) -> V {
        value.trim();
        value
    }
}

impl Adjust<String> for Trim {
    fn adjust(value: &mut String) {
        value.trim()
    }
}

impl<V> TryTransform<V> for Trim
where
    V: TrimTrait,
{
    type Error = ();

    fn try_transform(mut value: V) -> Result<V, Self::Error> {
        value.trim();
        Ok(value)
    }
}
