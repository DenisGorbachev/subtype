use crate::errors::binary_error::BinaryError;

pub trait Validate<Value> {
    type Error;

    fn validate(value: &Value) -> Option<Self::Error>;
}

impl<A, B, ErrorA, ErrorB, Value> Validate<Value> for (A, B)
where
    A: Validate<Value, Error = ErrorA>,
    B: Validate<Value, Error = ErrorB>,
{
    type Error = BinaryError<ErrorA, ErrorB>;

    fn validate(value: &Value) -> Option<Self::Error> {
        None.or_else(|| A::validate(value).map(BinaryError::VariantA))
            .or_else(|| B::validate(value).map(BinaryError::VariantB))
    }
}

#[macro_export]
macro_rules! validate_as_check {
    (impl$([$($generics:tt)*])? Validate<$target:ty> for $checker:ty $(where [$($where_clause:tt)*])?) => {
        impl$(<$($generics)*>)? $crate::traits::validate::Validate<$target> for $checker where $($($where_clause)*)* {
            type Error = $crate::errors::ValidationError<$checker>;

            fn validate(value: &$target) -> Option<Self::Error> {
                if <$checker as $crate::traits::check::Check<$target>>::check(value) {
                    None
                } else {
                    Some(Self::Error::new())
                }
            }
        }
    };
}
