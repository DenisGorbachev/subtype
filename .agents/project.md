# Project description

## Concepts

### Newtype

A struct that contains exactly one data-carrying field.

Examples:

- [Name](#name)
- [DurationSeconds](#durationseconds)
- [Id](#id)

Requirements:

- Must have `#[repr(transparent)]`
- Must derive `Deref`, `Borrow` via `derive_more`
- Must derive `AsRef` via `derive_more`
  - Requirements:
    - Must have `#[as_ref(forward)]`
  - Reasons:
    - Lots of functions in `std` accept `AsRef<Path>`, so [AbsolutePathBuf](#absolutepathbuf) must implement `AsRef<Path>`
- If newtype is [refined](#refined-newtype):
  - Then:
    - Must have a doc comment
      - Must contain a `## Safety` section that describes the invariants
    - Must have a private visibility of the inner value
    - Must not provide mutable access to inner value
      - Must not implement `DerefMut`
    - Must have at least one `impl TryFrom`
      - Must not call `Self::new`
    - Must have functions:
      - `pub fn new`
        - Must call the primary `TryFrom`
      - `pub unsafe fn new_unchecked`
        - Must construct the value without safety checks
      - `pub fn set`
        - Must accept `&mut self`
        - Must replace `self` with a value received from `TryFrom`
  - Else:
    - If the inner field is semantically mutable:
      - Then:
        - Must have a public visibility of the inner value
        - Must implement `DerefMut`, `BorrowMut`
      - Else:
        - Must have a private visibility of the inner value
        - Must not implement `DerefMut`, `BorrowMut`
    - Must have `impl From` for constructing the outer value
- If the crate contains a `serde` dependency, then:
  - Must have `Serialize` and `Deserialize` impls
    - Notes:
      - The impls may be derived or custom
      - The impls may be feature-gated if the `serde` dependency is optional
      - The derives may be spelled as `serde::Serialize` or just `Serialize`, depending on whether other items with "Serialize" name are used (e.g. `rkyv::Serialize`)
  - Must have `#[serde(transparent)]`
    - Notes:
      - This attribute is necessary for `Serialize`, and it doesn't conflict with `#[serde(try_from = I)]`
  - If newtype has a `Deserialize` derive:
    - If newtype is [refined](#refined-newtype), then:
      - Must have `#[serde(try_from = I)]` (`I` is the inner type)

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
- Release locks / free the resources in a `Drop` impl
  - Examples:
    - Enforce that a `RawFd` is closed when `File` goes out of scope
- Define impls of foreign traits for foreign types
- Define impls that are different from existing impls for the inner type
  - Examples:
    - Define `impl Serialize` and `impl Deserialize` for newtype of `solana_address::Address` that serialize/deserialize the address as string (not byte array)
    - Define `impl Deserialize` for newtype of `rust_bitcoin::Address` that calls `assume_checked`

Counter-purposes:

- "Define methods on inner type" - it's better to implement traits or define free functions

### Mutable newtype

A newtype whose inner value can be mutated.

### Refined newtype

A newtype that enforces invariants at run-time.

Examples:

- [Name](#name)

Counter-examples:

- [DurationSeconds](#durationseconds)
- [Id](#id)

### Marked newtype

A newtype that contains an additional field for a generic argument `T`

Examples:

- [Id](#id)

Notes:

- A marked newtype can be raw or refined.

## Examples

### Name

`struct Name(String)`

Properties:

- [refined](#refined-newtype): true
- [marked](#marked-newtype): false

Purposes:

- Enforces a run-time invariant that the inner string is not empty.

### DurationSeconds

`struct DurationSeconds(pub u32)`

Properties:

- [refined](#refined-newtype): false
- [marked](#marked-newtype): false

Purposes:

- Enforces a compile-time check that multiplying two values of its type returns `DurationSecondsSquared` (not `DurationSeconds`).

### Id

`struct Id<T> { inner: u64, marker: PhantomData<T> }`

Properties:

- [refined](#refined-newtype): false
- [marked](#marked-newtype): true
- [mutable](#mutable-newtype): false

Aliases:

- `type UserId = Id<UserMarker>`
- `type OrderId = Id<OrderMarker>`

Purposes:

- Enforces a compile-time check that `UserId` is distinct from `OrderId`

Notes:

- Defining `Id` with aliases is better than defining `UserId`, `OrderId` and other ids as separate types:
  - Lower build time
  - Lower codebase size
  - Less trait impls

### AbsolutePathBuf

`struct AbsolutePathBuf(PathBuf)`
