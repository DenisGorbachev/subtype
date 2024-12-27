#[macro_export]
macro_rules! subtype_u64 {
    (
        $(#[$meta:meta])*
        $newvis:vis struct $newtype:ident(u64 $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*)$(;)?
    ) => {
        $crate::subtype_primitive_number!(
            $(#[$meta])*
            $newvis struct $newtype(u64 $([$preprocessor])* | $checker $([$postprocessor])*);
        );
    };
    (
        $(#[$meta:meta])*
        $newvis:vis struct $newtype:ident($oldvis:vis u64 $([$preprocessor:ty])*)$(;)?
    ) => {
        $crate::subtype_primitive_number!(
            $(#[$meta])*
            $newvis struct $newtype($oldvis u64 $([$preprocessor])*);
        );
    };
}

#[cfg(test)]
mod tests {
    use crate::Equal;
    use crate::U64;
    use assert_matches::assert_matches;

    subtype_u64! {
        pub struct SubtypeU64Plain(u64)
    }

    subtype_u64! {
        pub struct SubtypeU64Validated(u64 | Equal<U64<10>>)
    }

    #[test]
    fn must_reject_invalid_values() {
        assert_matches!(SubtypeU64Validated::try_from(0), Err(_));
    }

    #[test]
    fn must_allow_plain_values() {
        let a = SubtypeU64Plain::from(5);
        let b = SubtypeU64Plain::from(10);
        let c = a + b;
        assert_eq!(c, 15.into());
    }
}
