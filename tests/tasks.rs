use derive_builder::Builder;
use derive_more::From;

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

#[derive(Builder, Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Database {
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
