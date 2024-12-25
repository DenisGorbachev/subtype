use subtype::{newtype, Conjure, Equal};

newtype!(
    #[derive(PartialOrd, PartialEq, Clone, Debug)]
    pub struct Generic[T](Option<T>) where [T: Clone];
);

pub struct OptionNone;

impl<T> Conjure<Option<T>> for OptionNone {
    fn conjure() -> Option<T> {
        None
    }
}

newtype!(
    #[derive(PartialOrd, PartialEq, Clone, Debug)]
    pub struct OnlyNone[T](Option<T> | Equal<OptionNone>) where [T: Clone + Eq];
);

// TODO: Document that trait bounds for generics must be written in where clause
// newtype!(
//     #[derive(PartialOrd, PartialEq, Clone, Debug)]
//     pub struct Generic[T: Iterator](Option<T>);
// );
