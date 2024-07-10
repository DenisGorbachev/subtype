pub trait Transform<Value> {
    type Error;

    fn transform(value: Value) -> Result<Value, Self::Error>;
}

#[macro_export]
macro_rules! impl_transform_as_validate {
    ($validator:ty) => {
        impl<V> Transform<V> for $validator {
            type Error = <$validator as $crate::traits::validate::Validate<V>>::Error;

            fn transform(value: V) -> Result<V, Self::Error> {
                match <$validator as $crate::traits::validate::Validate<V>>::validate(&value) {
                    None => Ok(value),
                    Some(error) => Err(error),
                }
            }
        }
    };
}
