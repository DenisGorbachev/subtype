# Subtype

## Constraint types

* Constraint types must be empty structs (without fields)
* Constraint types must have the following derive attribute: `#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]`
  * `Default` is necessary to display the generic `ValidationError` (its `Display` impl requires `Checker: Display`)
  * Other traits are necessary to enable the same derives for newtypes

## Error types

* Error types must derive `Error` (obviously)
* Error types must take ownership of the value that triggered the error, so that the caller would have access to the value in an error handler
* Error type fields must be `pub`

# # Similar crates

* [refinement](https://crates.io/crates/refinement)
  * You can't foreign traits on a `Refinement` type (because the `Refinement` type comes from the `refinement` crate, which is also foreign to your code)
  * You can't sanitize the value in the constructor
* [nutype](https://crates.io/crates/nutype)
  * You can't return a specific error from the validation predicate
  * You have to wait longer due to increased compilation speed
  * You can't use autocompletion while writing out the macro call
* [synonym](https://crates.io/crates/synonym)
  * You can automatically get certain derives based on the underlying type
    * You can't get automatic derives for unknown types (for example, if `Address: Copy`, then `pub struct AddressNewtype(Address)` you won't get `impl Copy for AddressNewtype`, you need to explicitly add `#[derive(Copy)]`)
