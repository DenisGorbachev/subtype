#![allow(unreachable_code)]
#![allow(dead_code)]

use anyhow::bail;
use derive_builder::Builder;
use derive_more::From;
use time::OffsetDateTime;
use url::Url;
use url_macro::url;

use subtype::conjurers::now::Now;
use subtype::conjurers::u32::U32;
use subtype::newtype;
use subtype::{FieldEqual, LessThan};

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
    pub struct ExpectedTupleSize(u32 | FieldEqual<U32<12>>)
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
    error_types_unified: bool,
    validator_error_message_improved: bool,
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

macro_rules! guard_ok {
    ($expr:expr) => {
        if !$expr {
            return Ok(());
        }
    };
}

type Outcome<T = ()> = anyhow::Result<T>;

fn main() -> Outcome {
    implement_batteries_included_newtypes()?;
    guard_ok!(code_review_successful()?);
    release_the_crate()?;
    promote_the_crate()?;
    Ok(())
    // let database = DatabaseBuilder::default().build().unwrap();
    // let new = Node::new;
    // let tasks = vec![];
}

fn implement_batteries_included_newtypes() -> Outcome {
    let _names = [
        "email",
        "username", // disallow special characters
        "",
    ];
    bail!("implement_batteries_included_newtypes")
}

newtype!(
    pub struct StartedAt(OffsetDateTime | LessThan<Now>);
);

#[allow(dead_code)]
pub struct Mention {
    url: Url,
    status: Option<Status<bool>>,
}

pub enum Status<T> {
    Pending(StartedAt),
    Ready(T),
}

fn get_placement_urls() -> Vec<Url> {
    vec![
        url!("https://corrode.dev/blog/compile-time-invariants/")
    ]
}

fn promote_the_crate() -> Outcome {
    bail!("Leave comments on searches in Google");
    bail!("Get mentions on relevant urls");
}

fn code_review_successful() -> Outcome<bool> {
    todo!()
}

fn release_the_crate() -> Outcome {
    bail!("Release the crate")
}
