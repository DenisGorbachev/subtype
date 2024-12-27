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
