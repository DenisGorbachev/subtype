/// This macro doesn't include [`derive_more::Neg`] because it is also used for non-negative types that don't implement [`std::ops::Neg`] (e.g. `u32`)
#[macro_export]
macro_rules! newtype_primitive_number {
    (
        $(#[$meta:meta])*
        $newvis:vis struct $newtype:ident($oldtype:ty $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*)$(;)?
    ) => {
        $crate::newtype!(
            #[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
            // #[derive(Default)] is not present because it can bypass the validation
            #[derive(derive_more::Display)]
            // Numeric traits like Add, AddAssign, etc, are not derived because they can bypass the validation
            $(#[$meta])*
            $newvis struct $newtype($oldtype $([$preprocessor])* | $checker $([$postprocessor])*);
        );
    };
    (
        $(#[$meta:meta])*
        $newvis:vis struct $newtype:ident($oldvis:vis $oldtype:ty $([$preprocessor:ty])*)$(;)?
    ) => {
        $crate::newtype!(
            #[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
            #[derive(Default)]
            #[derive(derive_more::Display)]
            #[derive(derive_more::Add, derive_more::Sub, derive_more::Mul, derive_more::Div, derive_more::Rem, derive_more::Shr, derive_more::Shl)]
            $(#[$meta])*
            $newvis struct $newtype($oldvis $oldtype $([$preprocessor])*);
        );
    };
}
