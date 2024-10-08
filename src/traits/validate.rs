use crate::ValidationError2;

pub trait Validate<Value> {
    type Error;

    fn validate(value: &Value) -> Option<Self::Error>;
}

impl<A, B, ErrorA, ErrorB, Value> Validate<Value> for (A, B)
where
    A: Validate<Value, Error = ErrorA>,
    B: Validate<Value, Error = ErrorB>,
{
    type Error = ValidationError2<ErrorA, ErrorB>;

    fn validate(value: &Value) -> Option<Self::Error> {
        None.or_else(|| A::validate(value).map(ValidationError2::Variant1))
            .or_else(|| B::validate(value).map(ValidationError2::Variant2))
    }
}

#[macro_export]
macro_rules! validate_as_check {
    (impl$([$($generics:tt)*])? Validate<$target:ty> for $checker:ty $(where [$($where_clause:tt)*])?) => {
        impl$(<$($generics)*>)? $crate::Validate<$target> for $checker where $($($where_clause)*)* {
            type Error = $crate::ValidationError<$checker>;

            fn validate(value: &$target) -> Option<Self::Error> {
                if <$checker as $crate::Check<$target>>::check(value) {
                    None
                } else {
                    Some(Self::Error::new())
                }
            }
        }
    };
}
