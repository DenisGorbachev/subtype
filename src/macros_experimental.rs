#[macro_export]
macro_rules! constructor_from_transformer_checker {
    ($visibility:vis fn $name:ident, $oldtype:ty $([$transformer:ty])? | $checker:ty, $style:ident, $field:ident) => {
            $visibility fn $name($field: impl Into<$oldtype>) -> Result<Self, $crate::errors::InvalidValueError<$oldtype, $checker>> {
                let $field = $field.into();$(
                let $field = <$transformer as $crate::traits::transform::Transform<$oldtype>>::transform($field);)?
                if !<$checker as $crate::traits::check::Check<$oldtype>>::check(&$field) {
                    return Err($crate::errors::InvalidValueError::<$oldtype, $checker>::new($field));
                }
                Ok($crate::construct!(Self, $style, $field))
            }
    };
}
