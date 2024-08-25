#[macro_export]
macro_rules! newtype_u32 {
    (
        $(#[$meta:meta])*
        $newvis:vis struct $newtype:ident(u32 $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*)$(;)?
    ) => {
        $crate::newtype_primitive_number!(
            $(#[$meta])*
            $newvis struct $newtype(u32 $([$preprocessor])* | $checker $([$postprocessor])*);
        );
    };
    (
        $(#[$meta:meta])*
        $newvis:vis struct $newtype:ident($oldvis:vis u32 $([$preprocessor:ty])*)$(;)?
    ) => {
        $crate::newtype_primitive_number!(
            $(#[$meta])*
            $newvis struct $newtype($oldvis u32 $([$preprocessor])*);
        );
    };
}

#[cfg(test)]
mod tests {
    use crate::Equal;
    use crate::U32;
    use assert_matches::assert_matches;

    newtype_u32! {
        pub struct NewtypeU32Plain(u32)
    }

    newtype_u32! {
        pub struct NewtypeU32Validated(u32 | Equal<U32<10>>)
    }

    #[test]
    fn must_reject_invalid_values() {
        assert_matches!(NewtypeU32Validated::try_from(0), Err(_));
    }

    #[test]
    fn must_allow_plain_values() {
        let a = NewtypeU32Plain::from(5);
        let b = NewtypeU32Plain::from(10);
        let c = a + b;
        assert_eq!(c, 15.into());
    }
}
