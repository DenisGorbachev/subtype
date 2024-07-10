pub trait Validate<Value> {
    type Error;

    fn validate(value: &Value) -> Option<Self::Error>;
}

#[macro_export]
macro_rules! validate_as_check {
    (impl$(<$($generics:tt),*>)? Validate<$target:ty> for $checker:ty $(where $($where_clause:tt)*)?) => {
        impl<$($($generics),*)*> $crate::traits::validate::Validate<$target> for $checker where $($($where_clause)*)* {
            type Error = $crate::validation_error::ValidationError<$checker>;

            fn validate(value: &$target) -> Option<Self::Error> {
                if <$checker as $crate::traits::check::Check<$target>>::check(value) {
                    None
                } else {
                    Some($crate::validation_error::ValidationError::<$checker>::new())
                }
            }
        }
    };
}
