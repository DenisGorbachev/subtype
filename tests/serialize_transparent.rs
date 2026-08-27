#![cfg(feature = "macros")]

use core::convert::Infallible;
use serde_test::{Token, assert_ser_tokens};
use subtype::SerializeTransparent;

#[derive(SerializeTransparent)]
struct NonCloneTuple(String);

#[derive(SerializeTransparent)]
struct Named {
    value: u64,
}

#[derive(SerializeTransparent)]
struct Generic<T>(T)
where
    T: AsRef<str>;

#[test]
fn tuple_struct_must_serialize_as_its_field_without_clone() -> Result<(), Infallible> {
    assert_ser_tokens(&NonCloneTuple("value".to_string()), &[Token::Str("value")]);
    Ok(())
}

#[test]
fn named_struct_must_serialize_as_its_field() -> Result<(), Infallible> {
    assert_ser_tokens(
        &Named {
            value: 42,
        },
        &[Token::U64(42)],
    );
    Ok(())
}

#[test]
fn generic_struct_must_preserve_its_where_clause() -> Result<(), Infallible> {
    assert_ser_tokens(&Generic("value"), &[Token::Str("value")]);
    Ok(())
}
