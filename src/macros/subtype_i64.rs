#[macro_export]
macro_rules! subtype_i64 {
    (
        $(#[$meta:meta])*
        $newvis:vis struct $newtype:ident(i64 $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*)$(;)?
    ) => {
        $crate::subtype_primitive_number!(
            $(#[$meta])*
            $newvis struct $newtype(i64 $([$preprocessor])* | $checker $([$postprocessor])*);
        );
    };
    (
        $(#[$meta:meta])*
        $newvis:vis struct $newtype:ident($oldvis:vis i64 $([$preprocessor:ty])*)$(;)?
    ) => {
        $crate::subtype_primitive_number!(
            $(#[$meta])*
            $newvis struct $newtype($oldvis i64 $([$preprocessor])*);
        );
    };
}

#[cfg(test)]
mod tests {
    use crate::Equal;
    use crate::I64;
    use assert_matches::assert_matches;

    subtype_i64! {
        pub struct NewtypeI64Plain(i64)
    }

    subtype_i64! {
        pub struct NewtypeI64Validated(i64 | Equal<I64<10>>)
    }

    #[test]
    fn must_reject_invalid_values() {
        assert_matches!(NewtypeI64Validated::try_from(0), Err(_));
    }

    #[test]
    fn must_allow_plain_values() {
        let a = NewtypeI64Plain::from(-5);
        let b = NewtypeI64Plain::from(10);
        let c = a + b;
        assert_eq!(c, 5.into());
    }
}
