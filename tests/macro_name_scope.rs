use subtype::{construct, define};

macro_rules! test_subtype {
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

test_subtype!(Username, String, regular, value);
