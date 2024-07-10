pub trait Transform<Value> {
    type Error;

    fn transform(value: Value) -> Result<Value, Self::Error>;
}

#[macro_export]
macro_rules! impl_transform_as_validate {
    ($validator:ident) => {
        $crate::impl_transform_as_validate!($validator, <>);
    };
    ($validator:ident, <$($generics:ident),*>) => {
        $crate::impl_transform_as_validate!($validator, <$($generics),*>, where);
    };
    ($validator:ident, <$($generics:ident),*>, where $($where_clause:tt)*) => {
        impl<Value, $($generics),*> $crate::traits::transform::Transform<Value> for $validator<$($generics),*> where $($where_clause)* {
            type Error = <$validator<$($generics),*> as $crate::traits::validate::Validate<Value>>::Error;

            fn transform(value: Value) -> Result<Value, Self::Error> {
                match <$validator<$($generics),*> as $crate::traits::validate::Validate<Value>>::validate(&value) {
                    None => Ok(value),
                    Some(error) => Err(error),
                }
            }
        }
    };
}
