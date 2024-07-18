// #[macro_export]
// macro_rules! newtype {
//     ($(#[$meta:meta])* $visibility:vis struct $newtype:ident$([$($generics:tt)*])?($oldtype:ty) $(where $($where_clause:tt)*)?$(;)?) => {
//         $(#[$meta])*
//         $visibility struct $newtype$(<$($generics)*>)?($oldtype) $(where $($where_clause)*)?;
//     };
// }

#[macro_export]
macro_rules! newtype {
    // --- #[derive_auto]
    // (#[derive_auto] $(#[$meta:meta])* $visibility:vis struct $newtype:ident$([$($generics:tt)*])?($oldtype:ty | $transformer:ty) $(where [$($where_clause:tt)*])?$(;)?) => {
    //     $crate::newtype_derive_auto!(
    //         $(#[$meta])*
    //         $visibility struct $newtype$(<$($generics)*>)?($oldtype) $(where $($where_clause)*)?;
    //     );
    //
    //     $crate::impl_all_with_validation!(impl$(<$($generics)*>)? for $newtype $(where [$($where_clause)*])?, $transformer, $oldtype, tuple, value);
    // };
    // (#[derive_auto] $(#[$meta:meta])* $visibility:vis struct $newtype:ident$([$($generics:tt)*])? $(where [$($where_clause:tt)*])? { $field:ident: $oldtype:ty | $transformer:ty $(,)? }) => {
    //     $crate::newtype_derive_auto!(
    //         $(#[$meta])*
    //         $visibility struct $newtype$(<$($generics)*>)? $(where $($where_clause)*)? {
    //             $field: $oldtype
    //         }
    //     );
    //
    //     $crate::impl_all_with_validation!(impl$(<$($generics)*>)? for $newtype $(where [$($where_clause)*])?, $transformer, $oldtype, regular, $field);
    // };
    // (#[derive_auto] $(#[$meta:meta])* $visibility:vis struct $newtype:ident$([$($generics:tt)*])?($oldtype:ty) $(where [$($where_clause:tt)*])?$(;)?) => {
    //     $crate::newtype_derive_auto!(
    //         $(#[$meta])*
    //         $visibility struct $newtype$(<$($generics)*>)?($oldtype) $(where $($where_clause)*)?;
    //     );
    //
    //     $crate::impl_self_constructor_setter_without_validation!(impl$(<$($generics)*>)? for $newtype $(where [$($where_clause)*])?, $oldtype, tuple, value);
    // };
    // (#[derive_auto] $(#[$meta:meta])* $visibility:vis struct $newtype:ident$([$($generics:tt)*])? $(where [$($where_clause:tt)*])? { $field:ident: $oldtype:ty $(,)? }) => {
    //     $crate::newtype_derive_auto!(
    //         $(#[$meta])*
    //         $visibility struct $newtype$(<$($generics)*>)? $(where $($where_clause)*)? {
    //             $field: $oldtype
    //         }
    //     );
    //
    //     $crate::impl_self_constructor_setter_without_validation!(impl$(<$($generics)*>)? for $newtype $(where [$($where_clause)*])?, $oldtype, regular, $field);
    // };
    // --- #[not(derive_auto)]
    ($(#[$meta:meta])* $visibility:vis struct $newtype:ident$([$($generics:tt)*])?($oldtype:ty $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*) $(where [$($where_clause:tt)*])?$(;)?) => {
        $(#[$meta])*
        $visibility struct $newtype$(<$($generics)*>)?($oldtype) $(where $($where_clause)*)?;

        $crate::impl_all_with_validation!(impl$(<$($generics)*>)? for $newtype $(where [$($where_clause)*])?, $oldtype $([$preprocessor])* | $checker $([$postprocessor])*, tuple, value);
    };
    ($(#[$meta:meta])* $visibility:vis struct $newtype:ident$([$($generics:tt)*])? $(where [$($where_clause:tt)*])? { $field:ident: $oldtype:ty $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])* $(,)? }) => {
        $(#[$meta])*
        $visibility struct $newtype$(<$($generics)*>)? $(where $($where_clause)*)? {
            $field: $oldtype
        }

        $crate::impl_all_with_validation!(impl$(<$($generics)*>)? for $newtype $(where [$($where_clause)*])?, $oldtype $([$preprocessor])* | $checker $([$postprocessor])*, regular, $field);
    };
    ($(#[$meta:meta])* $visibility:vis struct $newtype:ident$([$($generics:tt)*])?($oldtype:ty) $(where [$($where_clause:tt)*])?$(;)?) => {
        $(#[$meta])*
        $visibility struct $newtype$(<$($generics)*>)?($oldtype) $(where $($where_clause)*)?;

        $crate::impl_all_without_validation!(impl$(<$($generics)*>)? for $newtype $(where [$($where_clause)*])?, $oldtype, tuple, value);
    };
    ($(#[$meta:meta])* $visibility:vis struct $newtype:ident$([$($generics:tt)*])? $(where [$($where_clause:tt)*])? { $field:ident: $oldtype:ty $(,)? }) => {
        $(#[$meta])*
        $visibility struct $newtype$(<$($generics)*>)? $(where $($where_clause)*)? {
            $field: $oldtype
        }

        $crate::impl_all_without_validation!(impl$(<$($generics)*>)? for $newtype $(where [$($where_clause)*])?, $oldtype, regular, $field);
    };
}

#[macro_export]
macro_rules! newtype_derive_auto {
    ($(#[$meta:meta])* $visibility:vis struct $newtype:ident(String)$(;)?) => {
        #[derive(Clone, Debug)]
        $(#[$meta])*
        $visibility struct $newtype(String);
    };
    ($(#[$meta:meta])* $visibility:vis struct $newtype:ident { $field:ident: String }$(;)?) => {
        #[derive(Clone, Debug)]
        $(#[$meta])*
        $visibility struct $newtype {
            $field: String
        }
    };
}

#[macro_export]
macro_rules! impl_all_with_validation {
    (impl$([$($generics:tt)*])? for $newtype:ty $(where [$($where_clause:tt)*])?, $oldtype:ty $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*, $style:ident, $field:ident) => {
        $crate::impl_self_constructor_setter_with_validation!(impl$(<$($generics)*>)? for $newtype $(where [$($where_clause)*])?, $oldtype $([$preprocessor])* | $checker $([$postprocessor])*, $style, $field, new, set);
        $crate::impl_try_from_own!(impl$(<$($generics)*>)? TryFrom<$oldtype> for $newtype $(where [$($where_clause)*])?, $crate::errors::InvalidValueError<$oldtype, $checker>, new);
        $crate::impl_try_from_ref!(impl$(<$($generics)*>)? TryFrom<&$oldtype> for $newtype $(where [$($where_clause)*])?, $crate::errors::InvalidValueError<$oldtype, $checker>, new, Clone::clone);
    };
}

#[macro_export]
macro_rules! impl_all_without_validation {
    (impl$([$($generics:tt)*])? for $newtype:ty $(where [$($where_clause:tt)*])?, $oldtype:ty, $style:ident, $field:ident) => {
        $crate::impl_self_constructor_setter_without_validation!(impl$(<$($generics)*>)? for $newtype $(where [$($where_clause)*])?, $oldtype, $style, $field, new, set);
        $crate::impl_from_own!(impl$(<$($generics)*>)? From<$oldtype> for $newtype $(where [$($where_clause)*])?, new);
        $crate::impl_from_ref!(impl$(<$($generics)*>)? From<&$oldtype> for $newtype $(where [$($where_clause)*])?, new, Clone::clone);
    };
}

#[macro_export]
macro_rules! impl_self_constructor_setter_with_validation {
    (impl$([$($generics:tt)*])? for $newtype:ty $(where [$($where_clause:tt)*])?, $oldtype:ty $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*, $style:ident, $field:ident, $constructor_method:ident, $setter_method:ident) => {
        impl$(<$($generics)*>)? $newtype $(where [$($where_clause)*])? {
            $crate::constructor_with_validation!(pub fn $constructor_method, $oldtype $([$preprocessor])* | $checker $([$postprocessor])*, $style, $field);
            $crate::setter_with_validation!(pub fn $setter_method, $oldtype $([$preprocessor])* | $checker $([$postprocessor])*, $style, $field);
        }
    }
}

#[macro_export]
macro_rules! impl_self_constructor_setter_without_validation {
    (impl$([$($generics:tt)*])? for $newtype:ty $(where [$($where_clause:tt)*])?, $oldtype:ty, $style:ident, $field:ident, $constructor_method:ident, $setter_method:ident) => {
        impl$(<$($generics)*>)? $newtype $(where [$($where_clause)*])? {
            $crate::constructor_without_validation!(pub fn $constructor_method, $oldtype, $style, $field);
            $crate::setter_without_validation!(pub fn $setter_method, $oldtype, $style, $field);
        }
    }
}

/// There can be only one checker because the function returns a single `Result<Self, Error>` and the `Error` type can support only one checker
#[macro_export]
macro_rules! constructor_with_validation {
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
    ($visibility:vis fn $name:ident, $oldtype:ty $([$preprocessor:ty])* $(| $checker:ty)+ { $error:ty } $([$postprocessor:ty])*, $style:ident, $field:ident) => {
            $visibility fn $name($field: impl Into<$oldtype>) -> Result<Self, $error> {
                let $field = $field.into();$(
                let $field = <$preprocessor as $crate::traits::transform::Transform<$oldtype>>::transform($field);)*
                $(if !<$checker as $crate::traits::check::Check<$oldtype>>::check(&$field) {
                    return Err($crate::errors::InvalidValueError::<$oldtype, $checker>::new($field).into());
                })+$(
                let $field = <$postprocessor as $crate::traits::transform::Transform<$oldtype>>::transform($field);)*
                Ok($crate::construct!(Self, $style, $field))
            }
    };
}

#[macro_export]
macro_rules! constructor_without_validation {
    ($visibility:vis fn $name:ident, $oldtype:ty, $style:ident, $field:ident) => {
            $visibility fn $name($field: impl Into<$oldtype>) -> Self {
                let $field = $field.into();
                $crate::construct!(Self, $style, $field)
            }
    };
}

#[macro_export]
macro_rules! setter_with_validation {
    ($visibility:vis fn $name:ident, $oldtype:ty $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*, $style:ident, $field:ident) => {
            $visibility fn $name(&mut self, $field: impl Into<$oldtype>) -> Result<(), $crate::errors::InvalidValueError<$oldtype, $checker>> {
                let $field = $field.into();$(
                let $field = <$preprocessor as $crate::traits::transform::Transform<$oldtype>>::transform($field);)*
                if !<$checker as $crate::traits::check::Check<$oldtype>>::check(&$field) {
                    return Err($crate::errors::InvalidValueError::<$oldtype, $checker>::new($field));
                }$(
                let $field = <$postprocessor as $crate::traits::transform::Transform<$oldtype>>::transform($field);)*
                $crate::assign!(self, $style, $field);
                Ok(())
            }
    };
}

#[macro_export]
macro_rules! setter_without_validation {
    ($visibility:vis fn $name:ident, $oldtype:ty, $style:ident, $field:ident) => {
            $visibility fn $name(&mut self, $field: impl Into<$oldtype>) {
                let $field = $field.into();
                $crate::assign!(self, $style, $field);
            }
    };
}

#[macro_export]
macro_rules! impl_try_from_own {
    (impl$([$($generics:tt)*])? TryFrom<$oldtype:ty> for $newtype:ty $(where [$($where_clause:tt)*])?, $error:ty, $method:ident $(, $wrapper:expr)?) => {
        impl$(<$($generics)*>)? TryFrom<$oldtype> for $newtype $(where $($where_clause)*)? {
            type Error = $error;

            fn try_from(value: $oldtype) -> Result<Self, Self::Error> {
                Self::$method($($wrapper)?(value))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_try_from_ref {
    (impl$([$($generics:tt)*])? TryFrom<&$oldtype:ty> for $newtype:ty $(where [$($where_clause:tt)*])?, $error:ty, $method:ident $(, $wrapper:expr)?) => {
        impl$(<$($generics)*>)? TryFrom<&$oldtype> for $newtype $(where $($where_clause)*)? {
            type Error = $error;

            fn try_from(value: &$oldtype) -> Result<Self, Self::Error> {
                Self::$method($($wrapper)?(value))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_from_own {
    (impl$([$($generics:tt)*])? From<$oldtype:ty> for $newtype:ty $(where [$($where_clause:tt)*])?, $method:ident $(, $wrapper:expr)?) => {
        impl$(<$($generics)*>)? From<$oldtype> for $newtype $(where $($where_clause)*)? {
            fn from(value: $oldtype) -> Self {
                Self::$method($($wrapper)?(value))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_from_ref {
    (impl$([$($generics:tt)*])? From<&$oldtype:ty> for $newtype:ty $(where [$($where_clause:tt)*])?, $method:ident $(, $wrapper:expr)?) => {
        impl$(<$($generics)*>)? From<&$oldtype> for $newtype $(where $($where_clause)*)? {
            fn from(value: &$oldtype) -> Self {
                Self::$method($($wrapper)?(value))
            }
        }
    };
}
