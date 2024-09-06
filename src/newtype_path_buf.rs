#[macro_export]
macro_rules! newtype_path_buf {
    (
        $(#[$meta:meta])*
        $visibility:vis struct $newtype:ident(PathBuf $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*)$(;)?
    ) => {
        $crate::newtype!(
            #[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Debug)]
            // #[derive(Default)] is not present because it can bypass the validation
            $(#[$meta])*
            $visibility struct $newtype(std::path::PathBuf $([$preprocessor])* | $checker $([$postprocessor])*);
        );
        $crate::impl_try_from_ref!(impl TryFrom<&std::path::Path> for $newtype, $crate::IncorrectValueError<std::path::PathBuf, <$checker as $crate::Validate<std::path::PathBuf>>::Error>, new, ToOwned::to_owned);
        $crate::impl_as_ref_delegate!(impl AsRef<std::path::Path> for $newtype);
    };
    (
        $(#[$meta:meta])*
        $visibility:vis struct $newtype:ident($oldvis:vis PathBuf $([$preprocessor:ty])*)$(;)?
    ) => {
        $crate::newtype!(
            #[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Debug)]
            #[derive(Default)]
            $(#[$meta])*
            $visibility struct $newtype($oldvis std::path::PathBuf $([$preprocessor])*);
        );
        $crate::impl_from_ref!(impl From<&std::path::Path> for $newtype, new, ToOwned::to_owned);
        $crate::impl_as_ref_delegate!(impl AsRef<std::path::Path> for $newtype);
    };
}

#[cfg(test)]
mod tests {
    use crate::{Empty, Not};

    newtype_path_buf! {
        pub struct NewtypePlain(PathBuf)
    }

    newtype_path_buf! {
        pub struct NewtypeWithChecker(PathBuf | Not<Empty>)
    }
}
