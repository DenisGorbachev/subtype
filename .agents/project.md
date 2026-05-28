# Subtype concepts

## Newtype

A struct that contains exactly one data-carrying field.

Examples:

- `Name(String)` enforces a run-time invariant that the inner string is not empty.
- `DurationSeconds(pub u32)` provides a compile-time guarantee that multiplying two values of its type returns `DurationSecondsSquared` (not `DurationSeconds`).

Purposes:

- Enforce invariants at run-time
  - Examples:
    - Enforce that a `String` is not empty
    - Enforce that a `u64` is not zero
    - Enforce that a `Vec` is sorted
- Enforce compile-time checks
  - Examples:
    - Enforce that only values with the same unit can be added or subtracted
    - Enforce that ids of objects in different collections cannot be used interchangeably
      - `Id<User>` cannot be used instead of `Id<Order>`
- Enforce cleanup of resources in a `Drop` impl
  - Examples:
    - Enforce that a `RawFd` is closed when `File` goes out of scope
- Define impls of foreign traits for foreign types
- Define impls that are different from existing impls for the inner type
  - Examples:
    - Define `impl Serialize` and `impl Deserialize` for newtype of `solana_address::Address` that serialize/deserialize the address as string (not byte array)
    - Define `impl Deserialize` for newtype of `rust_bitcoin::Address` that calls `assume_checked`

## Raw newtype

A newtype that doesn't enforce invariants at run-time.

Examples:

- `DurationSeconds(pub u32)`

Requirements:

- Must have a public visibility of the inner value
  - This also provides mutable access to the inner value
- Must implement `Deref`, `DerefMut`, `Borrow`, `BorrowMut`
- Should have `impl From` for constructing the outer value

Decisions:

- Should implement `AsRef`?
  - Facts:
    - `Borrow` is better than `AsRef` in this case
    - Some functions may have an `AsRef` bound, not `Borrow` bound
      - Lots of functions in `std` accept `AsRef<Path>`,

## Refined newtype

A newtype that enforces invariants at run-time.

Examples:

- `Name(String)`

Requirements:

- Must have a private visibility of the inner value
- Must not provide mutable access to inner value
  - Must not implement `DerefMut`
- Must have at least one `impl TryFrom`
- Must have functions:
  - `pub fn new`
    - Must call the primary `TryFrom`
  - `pub unsafe fn new_unchecked`
    - Must construct the value without safety checks
  - `pub fn set`
    - Must accept `&mut self`
    - Must replace `self` with a value received from `TryFrom`

## Marked newtype

A newtype that contains an additional field for a generic argument `T`

Examples:

- `struct Id<T> { pub inner: u64, pub marker: PhantomData<T> }`
  - Examples:
    - `type UserId = Id<UserMarker>;`
    - `type OrderId = Id<OrderMarker>;`
  - Reasons:
    - Reduces the build time
    - Reduces the code size
    - Allows to implement traits
  - Notes:
    - It is safe to have `pub` fields because the `Id` does not enforce run-time invariants

Notes:

- A marked newtype can be raw or refined.
