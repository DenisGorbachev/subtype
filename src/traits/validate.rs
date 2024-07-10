pub trait Validate<Value> {
    type Error;

    fn validate(value: &Value) -> Option<Self::Error>;
}

#[macro_export]
macro_rules! impl_validate_as_check {
    ($checker:ty) => {
        impl<Value> $crate::traits::validate::Validate<Value> for $checker {
            type Error = $crate::validation_error::ValidationError<$checker>;

            fn validate(value: &Value) -> Option<Self::Error> {
                if <$checker as $crate::traits::check::Check<Value>>::check(value) {
                    None
                } else {
                    Some($crate::validation_error::ValidationError::<$checker>::new())
                }
            }
        }
    };
}
