#[macro_export]
macro_rules! newtype_string {
    (
        $(#[$meta:meta])*
        $visibility:vis struct $newtype:ident(String $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*)$(;)?
    ) => {
        $crate::newtype!(
            #[derive(derive_more::Display, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Debug)]
            $(#[$meta])*
            $visibility struct $newtype(String $([$preprocessor])* | $checker $([$postprocessor])*);
        );
        $crate::impl_try_from_ref!(impl TryFrom<&str> for $newtype, $crate::errors::IncorrectValueError<String, <$checker as $crate::traits::validate::Validate<String>>::Error>, new, ToOwned::to_owned);
    };
    (
        $(#[$meta:meta])*
        $visibility:vis struct $newtype:ident(String $([$preprocessor:ty])*)$(;)?
    ) => {
        $crate::newtype!(
            #[derive(derive_more::Display, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Debug)]
            $(#[$meta])*
            $visibility struct $newtype(String $([$preprocessor])*);
        );
        $crate::impl_from_ref!(impl From<&str> for $newtype, new, ToOwned::to_owned);
    };
}

#[macro_export]
macro_rules! newtype_static_str {
    (
        $(#[$meta:meta])*
        $visibility:vis struct $newtype:ident(&'static str $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*)$(;)?
    ) => {
        $crate::newtype!(
            #[derive(derive_more::Display, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
            $(#[$meta])*
            $visibility struct $newtype(&'static str $([$preprocessor])* | $checker $([$postprocessor])*);
        );
    };
    (
        $(#[$meta:meta])*
        $visibility:vis struct $newtype:ident(&'static str $([$preprocessor:ty])*)$(;)?
    ) => {
        $crate::newtype!(
            #[derive(derive_more::Display, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
            $(#[$meta])*
            $visibility struct $newtype(&'static str $([$preprocessor])*);
        );
    };
}

#[cfg(test)]
mod tests {
    use crate::checkers::{Empty, Not};

    newtype_string! {
        pub struct NewtypeStringPlain(String)
    }

    newtype_string! {
        pub struct NewtypeStringChecker(String | Not<Empty>)
    }

    newtype_static_str! {
        pub struct A(&'static str)
    }

    newtype_static_str! {
        pub struct B(&'static str | Not<Empty>)
    }
}
