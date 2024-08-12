/// This macro doesn't include [`derive_more::Neg`] because it is also used for non-negative types that don't implement [`std::ops::Neg`] (e.g. `u32`)
#[macro_export]
macro_rules! newtype_primitive_number {
    (
        $(#[$meta:meta])*
        $visibility:vis struct $newtype:ident($oldtype:ty $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*)$(;)?
    ) => {
        $crate::newtype!(
            #[derive(derive_more::Display, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Debug)]
            // Numeric traits like Add, AddAssign, etc, are not derived because they can bypass the validation
            $(#[$meta])*
            $visibility struct $newtype($oldtype $([$preprocessor])* | $checker $([$postprocessor])*);
        );
    };
    (
        $(#[$meta:meta])*
        $visibility:vis struct $newtype:ident($oldtype:ty $([$preprocessor:ty])*)$(;)?
    ) => {
        $crate::newtype!(
            #[derive(derive_more::Display, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Debug)]
            #[derive(derive_more::Add, derive_more::Sub, derive_more::Mul, derive_more::Div, derive_more::Rem, derive_more::Shr, derive_more::Shl)]
            $(#[$meta])*
            $visibility struct $newtype($oldtype $([$preprocessor])*);
        );
    };
}
