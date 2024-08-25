#[macro_export]
macro_rules! newtype_static_str {
    (
        $(#[$meta:meta])*
        $visibility:vis struct $newtype:ident(&'static str $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*)$(;)?
    ) => {
        $crate::newtype!(
            #[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
            // #[derive(Default)] is not present because it can bypass the validation
            #[derive(derive_more::Display)]
            $(#[$meta])*
            $visibility struct $newtype(&'static str $([$preprocessor])* | $checker $([$postprocessor])*);
        );
    };
    (
        $(#[$meta:meta])*
        $visibility:vis struct $newtype:ident($oldvis:vis &'static str $([$preprocessor:ty])*)$(;)?
    ) => {
        $crate::newtype!(
            #[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
            #[derive(Default)]
            #[derive(derive_more::Display)]
            $(#[$meta])*
            $visibility struct $newtype($oldvis &'static str $([$preprocessor])*);
        );
    };
}

#[cfg(test)]
mod tests {
    use crate::{Empty, Not};

    newtype_static_str! {
        pub struct A(&'static str)
    }

    newtype_static_str! {
        pub struct B(&'static str | Not<Empty>)
    }
}
