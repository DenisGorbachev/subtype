#[macro_export]
macro_rules! assign {
    ($self:ident, tuple, $field:ident) => {
        $self.0 = $field;
    };
    ($self:ident, regular, $field:ident) => {
        $self.$field = $field;
    };
}
