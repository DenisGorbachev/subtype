#[macro_export]
macro_rules! newtype_string {
    (
        $(#[$meta:meta])*
        $visibility:vis struct $newtype:ident(String $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*)$(;)?
    ) => {
        $crate::newtype!(
            #[derive(derive_more::Display, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Debug)]
            $(#[$meta])*
            $visibility struct $newtype(String $([$preprocessor])* | $checker $([$postprocessor])*)$(;)?
        );
    };
}
