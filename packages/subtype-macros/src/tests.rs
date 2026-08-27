use crate::expand_serialize_transparent;
use assertables::assert_contains;
use core::convert::Infallible;
use syn::{DeriveInput, parse_quote};

#[test]
fn must_reject_non_structs() -> Result<(), Infallible> {
    assert_expansion_error(
        parse_quote!(
            enum Invalid {}
        ),
        "SerializeTransparent does not support enums",
    );
    assert_expansion_error(parse_quote!(union Invalid { value: u8 }), "SerializeTransparent does not support unions");
    Ok(())
}

#[test]
fn must_reject_structs_without_exactly_one_field() -> Result<(), Infallible> {
    assert_expansion_error(
        parse_quote!(
            struct Invalid;
        ),
        "SerializeTransparent requires a struct containing exactly one field",
    );
    assert_expansion_error(
        parse_quote!(
            struct Invalid(u8, u8);
        ),
        "SerializeTransparent requires a struct containing exactly one field",
    );
    assert_expansion_error(
        parse_quote!(
            struct Invalid {
                first: u8,
                second: u8,
            }
        ),
        "SerializeTransparent requires a struct containing exactly one field",
    );
    Ok(())
}

fn assert_expansion_error(input: DeriveInput, message: &str) {
    let output = expand_serialize_transparent(input).to_string();
    assert_contains!(output, message);
}
