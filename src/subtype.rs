// #[macro_export]
// macro_rules! newtype {
//     ($(#[$meta:meta])* $visibility:vis struct $newtype:ident$(<$($generics:tt),*>)?($oldtype:ty) $(where $($where_clause:tt)*)?$(;)?) => {
//         $(#[$meta])*
//         $visibility struct $newtype$(<$($generics),*>)?($oldtype) $(where $($where_clause)*)?;
//     };
// }

#[macro_export]
macro_rules! newtype_with_validation {
    ($(#[$meta:meta])* $visibility:vis struct $newtype:ident$(<$($generics:tt),*>)?($oldtype:ty | $transformer:ty) $(where ($($where_clause:tt)*))?$(;)?) => {
        $(#[$meta])*
        $visibility struct $newtype$(<$($generics),*>)?($oldtype) $(where $($where_clause)*)?;

        impl_all!(impl$(<$($generics),*>)? for $newtype $(where ($($where_clause)*))?, $transformer, $oldtype, tuple, value);
    };
    ($(#[$meta:meta])* $visibility:vis struct $newtype:ident$(<$($generics:tt),*>)? $(where ($($where_clause:tt)*))? { $field:ident: $oldtype:ty | $transformer:ty }) => {
        $(#[$meta])*
        $visibility struct $newtype$(<$($generics),*>)?($oldtype) $(where $($where_clause)*)?;

        impl_all!(impl$(<$($generics),*>)? for $newtype $(where ($($where_clause)*))?, $transformer, $oldtype, regular, $field);
    };
}

#[macro_export]
macro_rules! newtype_without_validation {
    () => {};
}

#[macro_export]
macro_rules! newtype_special {
    ($(#[$meta:meta])* $visibility:vis struct $newtype:ident(String)$(;)?) => {
        #[derive(Clone, Debug)]
        $(#[$meta])*
        $visibility struct $newtype(String);
    }
}

#[macro_export]
macro_rules! impl_all {
    (impl$(<$($generics:tt),*>)? for $newtype:ty $(where ($($where_clause:tt)*))?, $transformer:ty, $oldtype:ty, $style:ident, $field:ident) => {
        $crate::impl_constructor_setter!(impl$(<$($generics),*>)? for $newtype $(where ($($where_clause)*))?, $transformer, $oldtype, $style, $field);
        $crate::impl_try_from_own!(impl$(<$($generics),*>)? TryFrom<$oldtype> for $newtype $(where ($($where_clause)*))?, <$transformer as $crate::traits::transform::Transform<$oldtype>>::Error, new);
        $crate::impl_try_from_ref_clone!(impl<'a $(, $($generics),*)?> TryFrom<&'a $oldtype> for $newtype $(where ($($where_clause)*))?, <$transformer as $crate::traits::transform::Transform<$oldtype>>::Error, new);
    };
}

#[macro_export]
macro_rules! impl_constructor_setter {
    (impl$(<$($generics:tt),*>)? for $newtype:ty $(where ($($where_clause:tt)*))?, $transformer:ty, $oldtype:ty, $style:ident, $field:ident) => {
        impl$(<$($generics),*>)? $newtype $(where ($($where_clause)*))? {
            $crate::constructor!(pub fn new, $transformer, $oldtype, $style, $field);
            $crate::setter!(pub fn set, $transformer, $oldtype, $style, $field);
        }
    }
}

#[macro_export]
macro_rules! constructor {
    ($visibility:vis fn $name:ident, $transformer:ty, $oldtype:ty, $style:ident, $field:ident) => {
            $visibility fn $name(value: impl Into<$oldtype>) -> Result<Self, <$transformer as $crate::traits::transform::Transform<$oldtype>>::Error> {
                let $field = <$transformer as $crate::traits::transform::Transform<$oldtype>>::transform(value.into())?;
                Ok($crate::construct!(Self, $style, $field))
            }
    };
}

#[macro_export]
macro_rules! setter {
    ($visibility:vis fn $name:ident, $transformer:ty, $oldtype:ty, $style:ident, $field:ident) => {
            $visibility fn $name(&mut self, value: impl Into<$oldtype>) -> Result<(), <$transformer as $crate::traits::transform::Transform<$oldtype>>::Error> {
                let $field = <$transformer as $crate::traits::transform::Transform<$oldtype>>::transform(value.into())?;
                $crate::assign!(self, $style, $field);
                Ok(())
            }
    };
}

#[macro_export]
macro_rules! impl_try_from_own {
    (impl$(<$($generics:tt),*>)? TryFrom<$oldtype:ty> for $newtype:ty $(where ($($where_clause:tt)*))?, $error:ty, $method:ident $(, $wrapper:expr)?) => {
        impl$(<$($generics),*>)? TryFrom<$oldtype> for $newtype $(where $($where_clause)*)? {
            type Error = $error;

            fn try_from(value: $oldtype) -> Result<Self, Self::Error> {
                Self::$method($($wrapper)?(value))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_try_from_ref_clone {
    (impl<'a $(, $($generics:tt),*)?> TryFrom<&'a $oldtype:ty> for $newtype:ty $(where ($($where_clause:tt)*))?, $error:ty, $method:ident $(, $wrapper:expr)?) => {
        impl<'a $(, $($generics),*)?> TryFrom<&'a $oldtype> for $newtype $(where $($where_clause)*)? {
            type Error = $error;

            fn try_from(value: &'a $oldtype) -> Result<Self, Self::Error> {
                Self::$method(value.clone())
            }
        }
    };
}
