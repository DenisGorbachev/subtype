use crate::errors::InvalidValueError;

pub trait TryTransform<Value> {
    type Error;

    fn try_transform(value: Value) -> Result<Value, Self::Error>;
}

impl<A, B, Value> TryTransform<Value> for (A, B)
where
    A: TryTransform<Value, Error = InvalidValueError<Value, A>>,
    B: TryTransform<Value, Error = InvalidValueError<Value, A>>,
{
    type Error = InvalidValueError<Value, (A, B)>;

    fn try_transform(value: Value) -> Result<Value, Self::Error> {
        let value = A::try_transform(value).map_err(|error| Self::Error::new(error.value))?;
        let value = B::try_transform(value).map_err(|error| Self::Error::new(error.value))?;
        Ok(value)
    }
}

#[macro_export]
macro_rules! try_transform_as_validate {
    (impl$([$($generics:tt)*])? Transform<$target:ty> for $validator:ty $(where [$($where_clause:tt)*])?) => {
        impl$(<$($generics)*>)? $crate::TryTransform<$target> for $validator where $($($where_clause)*)* {
            type Error = <$validator as $crate::Validate<$target>>::Error;

            fn try_transform(value: $target) -> Result<$target, Self::Error> {
                match <$validator as $crate::Validate<$target>>::validate(&value) {
                    None => Ok(value),
                    Some(error) => Err(error),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! try_transform_as_check {
    (impl$([$($generics:tt)*])? Transform<$target:ty> for $checker:ty $(where [$($where_clause:tt)*])?) => {
        impl$(<$($generics)*>)? $crate::TryTransform<$target> for $checker where $($($where_clause)*)* {
            type Error = $crate::InvalidValueError<$target, $checker>;

            fn try_transform(value: $target) -> Result<$target, Self::Error> {
                if <$checker as $crate::Check<$target>>::check(&value) {
                    Ok(value)
                } else {
                    Err(Self::Error::new(value))
                }
            }
        }
    };
}
