use subtype::newtype;

newtype!(
    #[derive(PartialOrd, PartialEq, Clone, Debug)]
    pub struct Generic[T](Option<T>) where [T: Clone];
);

// TODO: Implement generics with trait bounds (or move trait bounds to where clause & document it)
// newtype!(
//     #[derive(PartialOrd, PartialEq, Clone, Debug)]
//     pub struct Generic[T: Iterator](Option<T>);
// );
