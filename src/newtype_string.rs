#[macro_export]
macro_rules! newtype_string {
    (
        $(#[$meta:meta])*
        $visibility:vis struct $newtype:ident(String $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*)$(;)?
    ) => {
        $crate::newtype!(
            #[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Debug)]
            // #[derive(Default)] is not present because it can bypass the validation
            #[derive(derive_more::Display)]
            $(#[$meta])*
            $visibility struct $newtype(String $([$preprocessor])* | $checker $([$postprocessor])*);
        );
        $crate::impl_try_from_ref!(impl TryFrom<&str> for $newtype, $crate::IncorrectValueError<String, <$checker as $crate::Validate<String>>::Error>, new, ToOwned::to_owned);
    };
    (
        $(#[$meta:meta])*
        $visibility:vis struct $newtype:ident($oldvis:vis String $([$preprocessor:ty])*)$(;)?
    ) => {
        $crate::newtype!(
            #[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Debug)]
            #[derive(Default)]
            #[derive(derive_more::Display)]
            $(#[$meta])*
            $visibility struct $newtype($oldvis String $([$preprocessor])*);
        );
        $crate::impl_from_ref!(impl From<&str> for $newtype, new, ToOwned::to_owned);
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
}
