#[macro_export]
macro_rules! newtype_u64 {
    (
        $(#[$meta:meta])*
        $newvis:vis struct $newtype:ident(u64 $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*)$(;)?
    ) => {
        $crate::newtype_primitive_number!(
            $(#[$meta])*
            $newvis struct $newtype(u64 $([$preprocessor])* | $checker $([$postprocessor])*);
        );
    };
    (
        $(#[$meta:meta])*
        $newvis:vis struct $newtype:ident($oldvis:vis u64 $([$preprocessor:ty])*)$(;)?
    ) => {
        $crate::newtype_primitive_number!(
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

    newtype_u64! {
        pub struct NewtypeU64Plain(u64)
    }

    newtype_u64! {
        pub struct NewtypeU64Validated(u64 | Equal<U64<10>>)
    }

    #[test]
    fn must_reject_invalid_values() {
        assert_matches!(NewtypeU64Validated::try_from(0), Err(_));
    }

    #[test]
    fn must_allow_plain_values() {
        let a = NewtypeU64Plain::from(5);
        let b = NewtypeU64Plain::from(10);
        let c = a + b;
        assert_eq!(c, 15.into());
    }
}
