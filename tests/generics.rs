use subtype::{subtype, Conjure, Equal, IncorrectValueError, ValidationError};

subtype!(
    #[derive(PartialOrd, PartialEq, Clone, Debug)]
    pub struct Generic[T](Option<T>) where [T: Clone];
);

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub struct OptionNone;

impl<T> Conjure<Option<T>> for OptionNone {
    fn conjure() -> Option<T> {
        None
    }
}

subtype!(
    #[derive(PartialOrd, PartialEq, Clone, Debug)]
    pub struct OnlyNone[T](Option<T> | Equal<OptionNone>) where [T: Clone + Eq];
);

// TODO: Document that trait bounds for generics must be written in where clause
// subtype!(
//     #[derive(PartialOrd, PartialEq, Clone, Debug)]
//     pub struct Generic[T: Iterator](Option<T>);
// );

#[test]
fn generics() {
    assert_eq!(Generic::new(Some(42)), Generic(Some(42)));
    assert_eq!(OnlyNone::<u64>::new(None), Ok(OnlyNone(None)));
    assert_eq!(OnlyNone::<u64>::new(Some(42)), Err(IncorrectValueError::new(Some(42), ValidationError::<Equal<OptionNone>>::new())));
}
