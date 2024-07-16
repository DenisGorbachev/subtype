# Development notes

## Tasks

* Try unifying the InvalidValueError and ValidationError
  * ValidationError = InvalidValueError<&'a Value> (pass a ref of Value to the InvalidValueError)

## Constraints

* Don't write a `impl<T> TryFrom<&T> for Username where T: ToOwned<Owned=String>`
  * This triggers a compiler error: ``conflicting implementation in crate `core`: impl<T, U> std::convert::TryFrom<U> for T where U: std::convert::Into<T>;``.
  * This error happens because there may be `impl Into<Username> for &A` and `impl ToOwned<Owned=String> for A`, which will transitively produce two conflicting `impl TryFrom<&A> for Username`
