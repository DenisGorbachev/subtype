#[macro_export]
macro_rules! subtype_hashmap {
    (
        $(#[$meta:meta])*
        $visibility:vis struct $newtype:ident(HashMap<$k:ty, $v:ty$(, $s:ty)?> $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*)$(;)?
    ) => {
        $crate::subtype!(
            #[derive(Eq, PartialEq, Clone, Debug)]
            // #[derive(Default)] is not present because it can bypass the validation
            $(#[$meta])*
            $visibility struct $newtype(std::collections::HashMap<$k, $v$(, $s)?> $([$preprocessor])* | $checker $([$postprocessor])*);
        );
    };
    (
        $(#[$meta:meta])*
        $visibility:vis struct $newtype:ident($oldvis:vis HashMap<$k:ty, $v:ty$(, $s:ty)?> $([$preprocessor:ty])*)$(;)?
    ) => {
        $crate::subtype!(
            #[derive(Eq, PartialEq, Clone, Debug)]
            #[derive(Default)]
            $(#[$meta])*
            $visibility struct $newtype($oldvis std::collections::HashMap<$k, $v$(, $s)?> $([$preprocessor])*);
        );
    };
}

#[cfg(test)]
mod tests {
    use crate::{IsEmpty, Not};

    subtype_hashmap! {
        pub struct HashMapStringString(HashMap<String, String>)
    }

    subtype_hashmap! {
        pub struct HashMapStringStringWithBuildHasher(HashMap<String, String, rustc_hash::FxBuildHasher>)
    }

    subtype_hashmap! {
        pub struct HashMapStringStringWithChecker(HashMap<String, String> | Not<IsEmpty>)
    }
}
