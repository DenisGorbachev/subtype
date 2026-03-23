#[macro_export]
macro_rules! subtype_i32 {
    (
        $(#[$meta:meta])*
        $newvis:vis struct $newtype:ident(i32 $([$preprocessor:ty])* | $checker:ty $([$postprocessor:ty])*)$(;)?
    ) => {
        $crate::subtype_primitive_number!(
            $(#[$meta])*
            $newvis struct $newtype(i32 $([$preprocessor])* | $checker $([$postprocessor])*);
        );
    };
    (
        $(#[$meta:meta])*
        $newvis:vis struct $newtype:ident($oldvis:vis i32 $([$preprocessor:ty])*)$(;)?
    ) => {
        $crate::subtype_primitive_number!(
            $(#[$meta])*
            $newvis struct $newtype($oldvis i32 $([$preprocessor])*);
        );
    };
}

#[cfg(test)]
mod tests {
    use crate::Equal;
    use crate::I32;
    use assert_matches::assert_matches;

    subtype_i32! {
        pub struct NewtypeI32Plain(i32)
    }

    subtype_i32! {
        pub struct NewtypeI32Validated(i32 | Equal<I32<10>>)
    }

    #[test]
    fn must_reject_invalid_values() {
        assert_matches!(NewtypeI32Validated::try_from(0), Err(_));
    }

    #[test]
    fn must_allow_plain_values() {
        let a = NewtypeI32Plain::from(-5);
        let b = NewtypeI32Plain::from(10);
        let c = a + b;
        assert_eq!(c, 5.into());
    }
}
