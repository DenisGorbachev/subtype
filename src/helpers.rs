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

#[macro_export]
macro_rules! construct {
    ($self:ident, tuple, $field:ident) => {
        $self($field)
    };
    ($self:ident, regular, $field:ident) => {
        $self {
            $field,
        }
    };
}

#[macro_export]
macro_rules! assign {
    ($self:ident, tuple, $field:ident) => {
        $self.0 = $field;
    };
    ($self:ident, regular, $field:ident) => {
        $self.$field = $field;
    };
}
