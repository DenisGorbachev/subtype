# Subtype concepts

## Newtype

A struct whose purpose is to enforce invariants at run-time and/or provide safety guarantees at compile-time.

Examples:

- `Name(String)` enforces a run-time invariant that the inner string is not empty.
- `DurationSeconds(pub u32)` provides a compile-time guarantee that multiplying two values of its type returns `DurationSecondsSquared` (not `DurationSeconds`).

## Raw newtype

A newtype that doesn't enforce invariants at run-time.

Examples:

- `DurationSeconds(pub u32)`

Requirements:

- Must have a public visibility of the inner value
- Should have `impl From` for constructing the outer value

## Refined newtype

A newtype that does enforce invariants at run-time.

Examples:

- `Name(String)`

Requirements:

- Must have a private visibility of the inner value
- Must have at least one `impl TryFrom`
- Must have functions:
  - `pub fn new`
    - Must call the primary `TryFrom`
  - `pub unsafe fn new_unchecked`
    - Must construct the value without safety checks
  - `pub fn set`
    - Must accept `&mut self`
    - Must replace `self` with a value received from `TryFrom`
