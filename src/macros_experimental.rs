/// There can be only one checker because the function returns a single `Result<Self, Error>` and the `Error` type can support only one checker
#[macro_export]
macro_rules! constructor_from_transformer_checker {
    ($visibility:vis fn $name:ident, $oldtype:ty $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*, $style:ident, $field:ident) => {
            $visibility fn $name($field: impl Into<$oldtype>) -> Result<Self, $crate::errors::InvalidValueError<$oldtype, $checker>> {
                let $field = $field.into();$(
                let $field = <$preprocessor as $crate::traits::transform::Transform<$oldtype>>::transform($field);)*
                if !<$checker as $crate::traits::check::Check<$oldtype>>::check(&$field) {
                    return Err($crate::errors::InvalidValueError::<$oldtype, $checker>::new($field));
                }$(
                let $field = <$postprocessor as $crate::traits::transform::Transform<$oldtype>>::transform($field);)*
                Ok($crate::construct!(Self, $style, $field))
            }
    };
}
