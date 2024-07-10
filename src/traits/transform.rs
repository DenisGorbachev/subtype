pub trait Transform<Value> {
    type Error;

    fn transform(value: Value) -> Result<Value, Self::Error>;
}

#[macro_export]
macro_rules! transform_as_validate {
    (impl$(<$($generics:tt),*>)? Transform<$target:ty> for $validator:ty $(where $($where_clause:tt)*)?) => {
        impl<$($($generics),*)*> $crate::traits::transform::Transform<$target> for $validator where $($($where_clause)*)* {
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
