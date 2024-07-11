#[macro_export]
macro_rules! impl_all {
    (impl$(<$($generics:tt),*>)? for $newtype:ty $(where $($where_clause:tt)*)?) => {
        impl$(<$($generics),*>)? $newtype $(where $($where_clause)*)? {}
    };
}

#[macro_export]
macro_rules! constructor {
    ($visibility:vis fn $name:ident, $transformer:ty, $oldtype:ty) => {
            $visibility fn $name(value: impl Into<$oldtype>) -> Result<Self, <$transformer as $crate::traits::transform::Transform<$oldtype>>::Error> {
                let value = <$transformer as $crate::traits::transform::Transform<$oldtype>>::transform(value.into())?;
                Ok(Self(value))
            }
    };
}

#[macro_export]
macro_rules! setter {
    ($visibility:vis fn $name:ident, $transformer:ty, $oldtype:ty) => {
            $visibility fn $name(&mut self, value: impl Into<$oldtype>) -> Result<(), <$transformer as $crate::traits::transform::Transform<$oldtype>>::Error> {
                let value = <$transformer as $crate::traits::transform::Transform<$oldtype>>::transform(value.into())?;
                self.0 = value;
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
macro_rules! impl_try_from_ref {
    (impl<'a $(, $($generics:tt),*)?> TryFrom<&'a $oldtype:ty> for $newtype:ty $(where ($($where_clause:tt)*))?, $error:ty, $method:ident $(, $wrapper:expr)?) => {
        impl<'a $(, $($generics),*)?> TryFrom<&'a $oldtype> for $newtype $(where $($where_clause)*)? {
            type Error = $error;

            fn try_from(value: &'a $oldtype) -> Result<Self, Self::Error> {
                Self::$method(value.clone())
            }
        }
    };
}
