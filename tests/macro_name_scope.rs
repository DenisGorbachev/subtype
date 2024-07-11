use subtype::{construct, define};

macro_rules! test_newtype {
    ($newtype:ident, $oldtype:ident, $style:ident, $field:ident) => {
        define!($newtype, $oldtype, $style, $field);

        #[allow(dead_code)]
        impl $newtype {
            pub fn new() -> Result<Self, ()> {
                let $field = "alice".to_string();
                Ok(construct!(Self, $style, $field))
            }
        }
    };
}

test_newtype!(Username, String, regular, value);

// newtype!(#[derive_auto] pub struct Hi(String););
