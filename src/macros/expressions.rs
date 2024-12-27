#[macro_export]
macro_rules! construct_tuple_struct {
    () => {
        Self(value)
    };
}

#[macro_export]
macro_rules! construct_regular_struct {
    (value) => {
        Self {
            value,
        }
    };
    ($field:ident) => {
        Self {
            $field: value,
        }
    };
}

#[macro_export]
macro_rules! assign_tuple_struct {
    () => {
        self.0 = value;
    };
}

#[macro_export]
macro_rules! assign_regular_struct {
    ($field:ident) => {
        self.$field = value;
    };
}
