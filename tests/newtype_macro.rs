use subtype::newtype_derive_auto;

newtype_derive_auto!(
    pub struct MyString(String);
);
