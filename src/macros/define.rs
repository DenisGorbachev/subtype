#[macro_export]
macro_rules! define {
    ($newtype:ident, $oldtype:ident, tuple, $field:ident) => {
        struct $newtype($oldtype);
    };
    ($newtype:ident, $oldtype:ident, regular, $field:ident) => {
        struct $newtype {
            $field: $oldtype,
        }
    };
}
