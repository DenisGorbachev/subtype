# Subtype

## Constraint types

* Constraint types must be empty structs (without fields)
* Constraint types must have the following derive attribute: `#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]`
  * `Default` is necessary to display the generic `ValidationError` (its `Display` impl requires `Checker: Display`)
  * Other traits are necessary to enable the same derives for newtypes

## Error types

* Error types must derive `Error` (obviously)
* Error types must take ownership of the value that triggered the error, so that the caller would have access to the value in an error handler
