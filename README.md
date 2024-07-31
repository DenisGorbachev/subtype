# Subtype

## Benefits

* You can implement custom validation
* You can use either tuple structs or regular structs (`struct Username(String)` or `struct Username { value: String }`)
* Good IDE support (e.g. rename, go to definition)

## Features

* Preprocessors are supported
* Postprocessors are supported

## Footguns

* It is possible to circumvent the validation by defining `impl From<String> for Username`. This is because an impl can construct `Username` directly without calling `Username::new`. There is no workaround for this, you just need to be careful.
* It is possible to circumvent the validation by adding `#[derive(serde::Deserialize)]`. This is because `impl serde::Deserialize` can construct the target type directly without calling the `new` function. There's a workaround: add `#[serde(try_from = "Foo")]` where Foo is the underlying type of this newtype.

## Gotchas

* In macro invocations, the generics and trait bounds must be wrapped in square brackets (`[]`) instead of angle brackets (`<>`). This is a limitation of macro_rules. It applies only to the _definitions_ of generics and trait bounds, not to their usage. See example:
  * Good: `validate_as_check!(impl[V] Validate<V> for Even where [V: IsEven]);`
  * Bad: `validate_as_check!(impl<V> Validate<V> for Even where V: IsEven);`

## Constraint types

* Constraint types must be empty structs (without fields)
* Constraint types must have the following derive attribute: `#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]`
  * `Default` is necessary to display the generic `ValidationError` (its `Display` impl requires `Checker: Display`)
  * Other traits are necessary to enable the same derives for newtypes

## Error types

* Error types must derive `Error` (obviously)
* Error types must take ownership of the value that triggered the error, so that the caller would have access to the value in an error handler
* Error type fields must be `pub`

## Design notes

* `Min<Minimum, Inclusivity>` is used instead of `Min<Minimum, const INCLUSIVITY: bool>` because it provides more informative error messages.
* `Min, Max, Equal` use `PartialOrd, PartialEq` instead of `Ord, Eq` because more types implement the Partial traits. Also, if the developer specifies `pub struct MyFloat(f32 | Min<0.5, Exclusive>)`, they don't want `NAN` to pass this validation. This is exactly what `Min` does by delegating to `PartialCmp`, which returns `false` for `NAN`.

# # Similar crates

* [refinement](https://crates.io/crates/refinement)
  * You can't implement foreign traits on a `Refinement` type (because the `Refinement` type comes from the `refinement` crate, which is also foreign to your code)
  * You can't sanitize the value in the constructor
* [nutype](https://crates.io/crates/nutype)
  * You can't return a specific error from the validation predicate
  * You have to wait longer due to increased compilation speed
  * You can't use autocompletion while writing out the macro call
* [synonym](https://crates.io/crates/synonym)
  * You can automatically get certain derives based on the underlying type
    * You can't get automatic derives for unknown types (for example, if `Address: Copy`, then `pub struct AddressNewtype(Address)` you won't get `impl Copy for AddressNewtype`, you need to explicitly add `#[derive(Copy)]`)
* [aliri_braid](https://crates.io/crates/aliri_braid)
  * It only supports `String` as the base type
* [prae](https://github.com/teenjuna/prae)
  * Very similar design
  * It's actually better
* [semval](https://github.com/slowtec/semval)
* [validators](https://crates.io/crates/validators)
