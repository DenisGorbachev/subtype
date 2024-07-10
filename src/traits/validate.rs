pub trait Validate<Value> {
    type Error;

    fn validate(value: &Value) -> Option<Self::Error>;
}

#[macro_export]
macro_rules! impl_validate_as_check {
    ($checker:ident) => {
        $crate::impl_validate_as_check!($checker, <>);
    };
    ($checker:ident, <$($generics:ident),*>) => {
        $crate::impl_validate_as_check!($checker, <$($generics),*>, where);
    };
    ($checker:ident, <$($generics:ident),*>, where $($where_clause:tt)*) => {
        impl<Value, $($generics),*> $crate::traits::validate::Validate<Value> for $checker<$($generics),*> where $($where_clause)* {
            type Error = $crate::validation_error::ValidationError<$checker<$($generics),*>>;

            fn validate(value: &Value) -> Option<Self::Error> {
                if <$checker<$($generics),*> as $crate::traits::check::Check<Value>>::check(value) {
                    None
                } else {
                    Some($crate::validation_error::ValidationError::<$checker<$($generics),*>>::new())
                }
            }
        }
    };
}
