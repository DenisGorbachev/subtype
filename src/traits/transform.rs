use crate::errors::BinaryError;

pub trait Transform<Value> {
    type Error;

    fn transform(value: Value) -> Result<Value, Self::Error>;
}

impl<A, B, ErrorA, ErrorB, Value> Transform<Value> for (A, B)
where
    A: Transform<Value, Error = ErrorA>,
    B: Transform<Value, Error = ErrorB>,
{
    type Error = BinaryError<ErrorA, ErrorB>;

    fn transform(value: Value) -> Result<Value, Self::Error> {
        let value = A::transform(value).map_err(BinaryError::VariantA)?;
        let value = B::transform(value).map_err(BinaryError::VariantB)?;
        Ok(value)
    }
}

#[macro_export]
macro_rules! transform_as_validate {
    (impl$([$($generics:tt)*])? Transform<$target:ty> for $validator:ty $(where [$($where_clause:tt)*])?) => {
        impl$(<$($generics)*>)? $crate::traits::transform::Transform<$target> for $validator where $($($where_clause)*)* {
            type Error = <$validator as $crate::traits::validate::Validate<$target>>::Error;

            fn transform(value: $target) -> Result<$target, Self::Error> {
                match <$validator as $crate::traits::validate::Validate<$target>>::validate(&value) {
                    None => Ok(value),
                    Some(error) => Err(error),
                }
            }
        }
    };
}
