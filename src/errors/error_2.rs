use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// This error type is used for tuples of checkers (which can return any error)
#[derive(derive_more::Error, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum ValidationError2<A, B> {
    Variant1(A),
    Variant2(B),
}

impl<A: Error, B: Error> Display for ValidationError2<A, B> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> ::core::fmt::Result {
        match self {
            ValidationError2::Variant1(err) => {
                if f.alternate() {
                    write!(f, "{:#}", err)
                } else {
                    write!(f, "{}", err)
                }
            }
            ValidationError2::Variant2(err) => {
                if f.alternate() {
                    write!(f, "{:#}", err)
                } else {
                    write!(f, "{}", err)
                }
            }
        }
    }
}
