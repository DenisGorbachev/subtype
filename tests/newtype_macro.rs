use subtype::newtype_special;

newtype_special!(
    pub struct MyString(String);
);

fn main() {}
