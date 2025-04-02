/// Cannot derive DerefMut because there might be a checker
#[macro_export]
macro_rules! subtype {
    // pub struct Username(String | Not<Empty>);
    (
        $(#[$meta:meta])*
        $newvis:vis struct $newtype:ident$([$($generics:tt)*])?($oldtype:ty $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*)
        $(where [$($where_clause:tt)*])?$(;)?
    ) => {
        #[derive(derive_more::Deref, derive_more::AsRef, derive_more::Into)]
        $(#[$meta])*
        $newvis struct $newtype$(<$($generics)*>)?($oldtype)
        $(where $($where_clause)*)?;

        $crate::impl_all_with_generic_validation!(impl$([$($generics)*])? for $newtype$(<$($generics)*>)? $(where [$($where_clause)*])?, $oldtype $([$preprocessor])* | $checker $([$postprocessor])*, tuple, value);
    };

    // pub struct Username(String);
    (
        $(#[$meta:meta])*
        $newvis:vis struct $newtype:ident$([$($generics:tt)*])?($oldvis:vis $oldtype:ty $([$preprocessor:ty])*)
        $(where [$($where_clause:tt)*])?$(;)?
    ) => {
        #[derive(derive_more::Deref, derive_more::DerefMut, derive_more::AsRef, derive_more::Into)]
        $(#[$meta])*
        $newvis struct $newtype$(<$($generics)*>)?($oldvis $oldtype) $(where $($where_clause)*)?;

        $crate::impl_all_without_validation!(impl$([$($generics)*])? for $newtype$(<$($generics)*>)? $(where [$($where_clause)*])?, $oldtype $([$preprocessor])*, tuple, value);
    };
}

#[macro_export]
macro_rules! impl_all_with_generic_validation {
    (impl$([$($generics:tt)*])? for $newtype:ty $(where [$($where_clause:tt)*])?, $oldtype:ty $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*, $style:ident, $field:ident) => {
        $crate::impl_self_constructor_setter_with_generic_validation!(impl$([$($generics)*])? for $newtype $(where [$($where_clause)*])?, $oldtype $([$preprocessor])* | $checker $([$postprocessor])*, $style, $field, new, set);
        $crate::impl_self_destructor!(impl$([$($generics)*])? for $newtype $(where [$($where_clause)*])?, $oldtype, into_inner);
        $crate::impl_try_from_own!(impl$([$($generics)*])? TryFrom<$oldtype> for $newtype $(where [$($where_clause)*])?, $crate::IncorrectValueError<$oldtype, <$checker as $crate::Validate<$oldtype>>::Error>, new);
        $crate::impl_try_from_ref!(impl$([$($generics)*])? TryFrom<&$oldtype> for $newtype $(where [$($where_clause)*])?, $crate::IncorrectValueError<$oldtype, <$checker as $crate::Validate<$oldtype>>::Error>, new, Clone::clone);
    };
}

#[macro_export]
macro_rules! impl_all_without_validation {
    (impl$([$($generics:tt)*])? for $newtype:ty $(where [$($where_clause:tt)*])?, $oldtype:ty $([$preprocessor:ty])*, $style:ident, $field:ident) => {
        $crate::impl_self_constructor_setter_without_validation!(impl$([$($generics)*])? for $newtype $(where [$($where_clause)*])?, $oldtype $([$preprocessor])*, $style, $field, new, set);
        $crate::impl_self_destructor!(impl$([$($generics)*])? for $newtype $(where [$($where_clause)*])?, $oldtype, into_inner);
        $crate::impl_from_own!(impl$([$($generics)*])? From<$oldtype> for $newtype $(where [$($where_clause)*])?, new);
        $crate::impl_from_ref!(impl$([$($generics)*])? From<&$oldtype> for $newtype $(where [$($where_clause)*])?, new, Clone::clone);
    };
}

#[macro_export]
macro_rules! impl_self_constructor_setter_with_generic_validation {
    (impl$([$($generics:tt)*])? for $newtype:ty $(where [$($where_clause:tt)*])?, $oldtype:ty $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*, $style:ident, $field:ident, $constructor_method:ident, $setter_method:ident) => {
        impl$(<$($generics)*>)? $newtype $(where $($where_clause)*)? {
            $crate::constructor_with_generic_validation!(pub fn $constructor_method, $oldtype $([$preprocessor])* | $checker $([$postprocessor])*, $style, $field);
            $crate::setter_with_generic_validation!(pub fn $setter_method, $oldtype $([$preprocessor])* | $checker $([$postprocessor])*, $style, $field);
        }
    }
}

#[macro_export]
macro_rules! impl_self_constructor_setter_without_validation {
    (impl$([$($generics:tt)*])? for $newtype:ty $(where [$($where_clause:tt)*])?, $oldtype:ty $([$preprocessor:ty])*, $style:ident, $field:ident, $constructor_method:ident, $setter_method:ident) => {
        impl$(<$($generics)*>)? $newtype $(where $($where_clause)*)? {
            $crate::constructor_without_validation!(pub fn $constructor_method, $oldtype $([$preprocessor])*, $style, $field);
            $crate::setter_without_validation!(pub fn $setter_method, $oldtype $([$preprocessor])*, $style, $field);
        }
    }
}

#[macro_export]
macro_rules! impl_self_destructor {
    (impl$([$($generics:tt)*])? for $newtype:ty $(where [$($where_clause:tt)*])?, $oldtype:ty, $destructor_method:ident) => {
        impl$(<$($generics)*>)? $newtype $(where $($where_clause)*)? {
            $crate::destructor!(pub fn $destructor_method, $oldtype);
        }
    }
}

// There can be only one checker because the function returns a single `Result<Self, Error>` and the `Error` type can support only one checker
// `constructor_with_custom_validation` is not needed because the user can simply implement a `TryFrom` trait (and also perform pre & post processing there) (but then the user wouldn't be able to use `set` method on the custom type (but it's rarely used))
// It's possible to implement helper macros to simplify the implementation of `TryFrom` that only calls a boolean-returning function
#[macro_export]
macro_rules! constructor_with_generic_validation {
    ($visibility:vis fn $name:ident, $oldtype:ty $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*, $style:ident, $field:ident) => {
            $visibility fn $name($field: impl Into<$oldtype>) -> Result<Self, $crate::IncorrectValueError<$oldtype, <$checker as $crate::Validate<$oldtype>>::Error>> {
                let $field = $field.into();$(
                let $field = <$preprocessor as $crate::Transform<$oldtype>>::transform($field);)*
                match <$checker as $crate::Validate<$oldtype>>::validate(&$field) {
                    None => {}
                    Some(err) => return Err($crate::IncorrectValueError::new($field, err))
                }$(
                let $field = <$postprocessor as $crate::Transform<$oldtype>>::transform($field);)*
                Ok($crate::construct!(Self, $style, $field))
            }
    };
}

#[macro_export]
macro_rules! constructor_without_validation {
    ($visibility:vis fn $name:ident, $oldtype:ty $([$preprocessor:ty])*, $style:ident, $field:ident) => {
            $visibility fn $name($field: impl Into<$oldtype>) -> Self {
                let $field = $field.into();$(
                let $field = <$preprocessor as $crate::Transform<$oldtype>>::transform($field);)*
                $crate::construct!(Self, $style, $field)
            }
    };
}

#[macro_export]
macro_rules! setter_with_generic_validation {
    ($visibility:vis fn $name:ident, $oldtype:ty $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*, $style:ident, $field:ident) => {
            $visibility fn $name(&mut self, $field: impl Into<$oldtype>) -> Result<(), $crate::IncorrectValueError<$oldtype, <$checker as $crate::Validate<$oldtype>>::Error>> {
                let $field = $field.into();$(
                let $field = <$preprocessor as $crate::Transform<$oldtype>>::transform($field);)*
                match <$checker as $crate::Validate<$oldtype>>::validate(&$field) {
                    None => {}
                    Some(err) => return Err($crate::IncorrectValueError::new($field, err))
                }$(
                let $field = <$postprocessor as $crate::Transform<$oldtype>>::transform($field);)*
                $crate::assign!(self, $style, $field);
                Ok(())
            }
    };
}

#[macro_export]
macro_rules! setter_without_validation {
    ($visibility:vis fn $name:ident, $oldtype:ty $([$preprocessor:ty])*, $style:ident, $field:ident) => {
            $visibility fn $name(&mut self, $field: impl Into<$oldtype>) {
                let $field = $field.into();$(
                let $field = <$preprocessor as $crate::Transform<$oldtype>>::transform($field);)*
                $crate::assign!(self, $style, $field);
            }
    };
}

#[macro_export]
macro_rules! destructor {
    ($visibility:vis fn $name:ident, $oldtype:ty) => {
            $visibility fn $name(self) -> $oldtype {
                self.0
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

#[macro_export]
macro_rules! impl_as_ref_delegate {
    (impl$([$($generics:tt)*])? AsRef<$sometype:ty> for $newtype:ty $(where [$($where_clause:tt)*])?) => {
        impl$(<$($generics)*>)? AsRef<$sometype> for $newtype $(where $($where_clause)*)? {
            fn as_ref(&self) -> &$sometype {
                self.0.as_ref()
            }
        }
    };
}
