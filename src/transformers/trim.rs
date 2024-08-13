use crate::traits::adjust::Adjust;
use crate::traits::transform::Transform;
use crate::traits::try_transform::TryTransform;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Trim;

impl<V> Transform<V> for Trim
where
    V: standard_traits::Trim,
{
    fn transform(mut value: V) -> V {
        value.trim();
        value
    }
}

impl Adjust<String> for Trim {
    fn adjust(value: &mut String) {
        standard_traits::Trim::trim(value)
    }
}

impl<V> TryTransform<V> for Trim
where
    V: standard_traits::Trim,
{
    type Error = ();

    fn try_transform(mut value: V) -> Result<V, Self::Error> {
        value.trim();
        Ok(value)
    }
}
