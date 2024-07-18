# Development notes

## Tasks

* Try unifying the InvalidValueError and ValidationError
  * ValidationError = InvalidValueError<&'a Value> (pass a ref of Value to the InvalidValueError)

## Constraints

* Don't write a `impl<T> TryFrom<&T> for Username where T: ToOwned<Owned=String>`
  * This triggers a compiler error: ``conflicting implementation in crate `core`: impl<T, U> std::convert::TryFrom<U> for T where U: std::convert::Into<T>;``.
  * This error happens because there may be `impl Into<Username> for &A` and `impl ToOwned<Owned=String> for A`, which will transitively produce two conflicting `impl TryFrom<&A> for Username`

## Questions

* How to avoid clones in `Not<Empty>` and `Or<A, B>` transformers?
  * Require every `Transformer` to return `Result<Value, (Value, Self::Error)>`
  * Require every `Transformer` to return `InvalidValueError` and mandate that the `value` field contains an unmodified value. In this case, we can extract this value in `Not`, so that we could avoid a clone
  * Provide a custom `NonEmpty` type which avoids a clone
  * Allow multiple trait calls in `pub fn new`, distinguish between Transformer and Checker calls, allow the user to write `| Trim % Not<Empty>`
* How to implement the generated `pub fn new`?
  * Options
    * Allow multiple trait calls + Require the user to supply the custom error type (because every trait call may return a `Result` with a different `Error` type, but `pub fn new` must return a `Result` with a single specific `Error` type)
    * Disallow multiple trait call + Specify the return `Error` type as `<Transformer as Transform>::Error`
