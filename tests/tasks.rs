use derive_builder::Builder;
use derive_more::From;

use subtype::constraints::Equal;
use subtype::containers::u32::U32;
use subtype::newtype;

// struct CanUnify<A, B>;
//
// pub type CanUnifyErrors = CanUnify<ValidationError<Empty>, InvalidValueError<String, Empty>>;

#[derive(From, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum ErrorsUnified {
    Yes,
    No,
}

#[derive(From, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum CanUnifyErrors {
    Yes(ErrorsUnified),
    No,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct SupportedOperators {
    not: bool,
    and: bool,
    or: bool,
}

newtype!(
    #[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
    pub struct ExpectedTupleSize(u32 | Equal<U32<12>>)
);

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct AndAsTuple {
    max_tuple_size: ExpectedTupleSize,
}

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct NotTransformFixed;

#[derive(Builder, Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Database {
    not_fixed: NotTransformFixed,
    supported_operators: SupportedOperators,
    and_as_tuple: AndAsTuple,
    can_unify_errors: CanUnifyErrors,
}

#[allow(dead_code)]
pub struct Node {
    title: String,
    children: Vec<Node>,
}

impl Node {
    pub fn new(title: impl Into<String>, children: impl IntoIterator<Item = impl Into<Node>>) -> Self {
        Self {
            title: title.into(),
            children: children.into_iter().map(|v| v.into()).collect(),
        }
    }
}

#[test]
fn tasks() {
    // let database = DatabaseBuilder::default().build().unwrap();
    // let new = Node::new;
    // let tasks = vec![];
}
