## Guidelines

### General

You are a senior Rust software architect.

- Think deeply and make detailed plans before writing the code.
- Write high-quality, production-ready, generic, reusable code.

#### Principles

Write code that minimizes losses:

- [Avoid data loss](#avoid-data-loss).
- [Minimize hardcoded data](#minimize-hardcoded-data).
- Minimize the memory consumption.
- Minimize the execution time.

##### Avoid data loss

- Don't use panicking functions (instead, use checked functions that return a `Result`)
- Don't delete the data unless the specification explicitly requires it

##### Minimize hardcoded data

- Don't hardcode the values (accept arguments instead)
- Choose carefully between accepting a parameter VS defining a constant:
  - Definitions:
    - Parameters are execution details (the user may want to change them)
    - Constants are implementation details (the user would never want to change them)
  - Examples:
    - Parameters:
      - Cache TTL
      - Config path
    - Constants:
      - Table name
      - Keyspace name
  - Recommendations:
    - When in doubt, prefer accepting a parameter instead of defining a constant

#### Development workflow

- After finishing the initial implementation, improve the code:
  - Remove unnecessary code
  - Remove unnecessary allocations
  - Refactor code that converts between types into `From` / `Into` impls
- After finishing the task, run `mise run agent:on:stop` (this command runs the lints and tests)
  - `mise run agent:on:stop` may modify `README.md`, `AGENTS.md`, `Cargo.toml` (this is normal, don't mention it)
  - `mise run agent:on:stop` includes `cargo fmt`, `cargo check`, `cargo clippy`, `cargo nextest` (no need to run them separately)
- Don't write tests
- Don't add comments
- Don't edit the files in `.agents`
- If a later instruction overrides the former instruction: follow the later instruction (last override wins)
- If I explicitly ask to update the code in a way that deviates from the spec, update both the code and the spec
- If you need to patch a dependency:
  - If the dependency is owned by Denis Gorbachev:
    - Then:
      - Find it in `~/workspace`
      - Apply edits
      - Add a temporary local `[patch]` in `.cargo/config.toml`
    - Else: tell me about it, but don't patch it without my explicit permission
- If you notice unexpected edits, keep them and don't mention them
- If you notice incorrect code, tell me
- If you have to apply a workaround, add a comment next to the workaround that explains why it is necessary, and also mention the workaround in your final report
- If the task can't be completed exactly as it is written (for example, due to limitations in the language or dependencies, or due to incorrect assumptions in the specification), append an item to [`findings.md`](#findingsmd) with priority `P0`.
- If unexpected behavior impedes your progress, but it's not a blocker (for example: domain is unavailable, program is unavailable, available memory or disk space is too low, command runs for unexpectedly long time or consumes an unexpected amount of resources), append an item to [`findings.md`](#findingsmd) with priority `P2`.
- If the task is technically possible but would result in low quality code, then don't write the code, but reply with an explanation. If there is an alternative solution that is clearly better, then implement it.
  - Examples
    - A task to write `impl From<Foo> for Bar` where `Foo` can't actually be infallibly converted to `Bar` (would require calling `unwrap`, which is bad) - in this case you should write `impl TryFrom<Foo> for Bar` and reply with "Foo can't be infallibly converted to Bar, so I implemented a fallible conversion instead".
    - A task to write a trait impl that only returns an error - in this case you should not write the trait impl but reply with "trait X can't be implemented for Foo because ..."
- If a sentence starts with "Idea: ":
  - Evaluate it thoroughly.
  - If you agree:
    - Then: implement it.
    - Else: explain why you didn't implement it and brainstorm solutions.
- If you resolve the findings, remove them from [findings.md](#findingsmd)
  - If [findings.md](#findingsmd) becomes empty, remove it

#### Review workflow

- Output a full list of [findings](#finding) (not a shortlist)
- If there are no findings, then start your reply with "No findings"
- If I reply to your review with an ordered list, process each item in the following way:
  - "+" - "Think about this finding again, then apply the best fix according to your thinking process"
  - "+ {number}" - "Apply proposed fix at {number}"
  - "-" - "Don't apply any fixes"
  - other - respond normally (keep the `ctid` in your response)
- If there are no more actionable items in the thread identified by a specific `ctid`: drop this `ctid` from your response.

#### Debugging workflow

- Improve error handling, so that the root cause is clearly visible

#### Subagents

- When spawning a code review subagent: use fresh context (not inherited).

#### Messages from agent to user

- Use `~` in paths.
- Format your message as a sequence of independently addressable items.
- Don't mention successful verifications and checks unless asked explicitly.

#### Commands

- Use `fd` and `rg` instead of `find` and `grep`
- Set the command-execution tool call’s timeout parameter and `yield_time_ms` to at least 300000 ms for the following commands: `mise run agent:on:stop`, `cargo build`, `git commit`

#### Recommended crates

- `errgonomic` for error handling
- `strum` for enum derives
- `subtype` for defining newtypes
- `tempfile` for creating temp dirs or files

#### Files

- The file name must match the name of the primary item in this file (for example: a file with `struct User` must be named `user.rs`)
- The trait implementations must be in the same file as the target type (for example: put `impl TryFrom<...> for User` in the same file as `struct User`, which is `user.rs`)

#### Modules

- Don't use `mod.rs`, use module files with submodules in the folder with the same name (for example: `user.rs` with submodules in `user` folder)
- When creating a new module, attach it with a `mod` declaration followed by `pub use` glob declaration. The parent module must re-export all items from the child modules. This allows to `use` the items right from the crate root, without intermediate module path. For example:
  ```rust
  fn foo() {}

  mod my_module_name;
  pub use my_module_name::*;
  ```
- Place the `mod` and `pub use` declarations at the end of the file (after the code items).
- When importing items that are defined in the current crate, use direct import from crate root. For example:
  ```rust
  use crate::foo;
  ```
- Prefer short item paths over long item paths (use `use` statement), unless it's necessary for disambiguation. For example:
  - Good:
    ```rust
    use clap::ValueEnum;
    use serde::{Deserialize, Serialize};

    #[derive(ValueEnum, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy, Debug)]
    pub enum Side {
        Buy,
        Sell,
    }
    ```
  - Good (`serde` and `rkyv` prefixes are necessary for disambiguation):
    ```rust
    use clap::ValueEnum;

    #[derive(ValueEnum, From, serde::Serialize, serde::Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Eq, PartialEq, Hash, Clone, Copy, Debug)]
    pub enum Side {
        Buy,
        Sell,
    }
    ```
  - Bad (`clap` and `serde` prefixes are not necessary for disambiguation because their trait names are unique in this module):
    ```rust
    #[derive(clap::ValueEnum, serde::Serialize, serde::Deserialize, Eq, PartialEq, Hash, Clone, Copy, Debug)]
    pub enum Side {
        Buy,
        Sell,
    }
    ```
- If you need error and result types from `std`, prefer short paths:
  - `use std::io;` and `io::Result`, `io::Error`
  - `use std::fmt;` and `fmt::Result`, `fmt::Error`

#### Visibility

- Items:
  - Prefer `pub` instead of `pub(crate)` or private.
- Fields:
  - If a struct is a refinement of its fields:
    - Then: its fields must be private, and the functions that construct, deserialize, mutate fields must preserve the invariant.
    - Else: its fields must be `pub`.

#### Constants

- Define constants only for values used in multiple places (prefer inline values)
- Put constants in `src/constants.rs`

#### Types

- Always use the most specific types (enforce semantic difference through syntactic difference):
  - Use types from existing crates
    - Use types from `url` crate instead of `String` for URL-related values
    - Use types from `time` crate instead of `String` or `u64` for datetime-related values
    - Use types from `phonenumber` crate instead of `String` for phone-related values
    - Use types from `email_address` crate instead of `String` for email-related values
    - Use types from `core::num` module that are prefixed with `NonZero` for values that must be non-zero
  - Search for other existing crates if you need specific types
  - If you can't find existing crates, define newtypes using macros from `subtype` crate
- Every `struct`, `enum`, `union` must be in a separate file (except for error types that implement `Error`)
  - Error types that implement `Error` must be in the same files as the functions that return them
- Prefer attaching the types as child modules to src/types.rs

#### Functions

- Prefer the weakest sufficient trait bound for inputs and associated types (`FnOnce` over `FnMut` over `Fn`) (`PartialOrd` over `Ord`) (`PartialEq` over `Eq`)
- Implement proper error handling using macros from `errgonomic` crate instead of `unwrap` or `expect` (in normal code and in tests)
  - Use `expect` only in exceptional cases where you can prove that it always succeeds, and provide the proof as the first argument to `expect` (the proof must start with "always succeeds because")
- Prefer streams and iterators:
  - Guidelines for inputs:
    - If the function uses methods that are available only for a specific collection type:
      - Then: prefer taking a specific collection type as input.
      - Else: prefer taking an `impl Stream` or `impl IntoIterator` as input.
  - Guidelines for outputs:
    - If the function return type is naturally an iterator (for example, the function returns the output of a `map` or `filter`):
      - Then: prefer returning an `impl Iterator` as output (there's no need to collect into `Vec`).
      - Else: prefer returning a specific collection type as output.
  - Examples:
    - Good:
      ```rust
      /// This is good because the function doesn't use any type-specific methods, only generic Iterator trait methods
      /// This is good because the function naturally returns an Iterator, not a specific collection type
      pub fn filter_non_empty_strings<'a>(inputs: impl IntoIterator<Item = &'a str>) -> impl Iterator<Item = &'a str> {
          inputs.into_iter().filter(|i| i.is_empty().not())
      }

      /// This is good because the function uses Vec-specific method `extend_from_slice`, so it can't take a generic `impl IntoIterator`
      fn extend_args(mut args: Vec<String>, extra_args: &[String]) -> Vec<String> {
          args.extend_from_slice(extra_args);
          args
      }
      ```
    - Bad:
    - ```rust
      /// This is bad because it needlessly converts a Vec into iter and then collects back into Vec
      pub fn filter_non_empty_strings(inputs: Vec<&str>) -> Vec<&str> {
          inputs
              .into_iter()
              .filter(|i| i.is_empty().not())
              .collect::<Vec<_>>()
      }

      /// This is bad because it is not general enough and also forces the caller to collect the strings into a vec for input, which is bad for performance
      pub fn bar(inputs: Vec<String>) -> Vec<String> {}
      ```
- Prefer implementing and use `From` or `TryFrom` for conversions between types (instead of converting in-place)
- Don't use early-return fast-path guards for empty vecs, iterators, streams (i.e. don't use `if items.is_empty() { return ...; }`)
- Use destructuring assignment for tuple arguments, for example: `fn try_from((name, parent_key): (&str, GroupKey)) -> ...`
- Use iterators instead of for loops. For example:
  - Good:
    ```rust
    use errgonomic::{handle_iter, ErrVec};
    use core::num::ParseIntError;
    use thiserror::Error;

    // Good: iterator pipeline with fallible mapping + correct error handling
    pub fn parse_numbers(inputs: impl IntoIterator<Item = impl AsRef<str>>) -> Result<Vec<u64>, ParseNumbersError> {
        use ParseNumbersError::*;
        let iter = inputs.into_iter().map(|s| s.as_ref().trim().parse::<u64>());
        Ok(handle_iter!(iter, InvalidInput))
    }

    #[derive(Error, Debug)]
    pub enum ParseNumbersError {
        #[error("failed to parse {len} numbers", len = source.len())]
        InvalidInput { source: ErrVec<ParseIntError> },
    }
    ```
  - Bad:
    ```rust
    use core::num::ParseIntError;

    // Bad: manual loop + mutable accumulator
    pub fn parse_numbers(inputs: impl IntoIterator<Item = impl AsRef<str>>) -> Result<Vec<u64>, ParseIntError> {
        let mut out = Vec::new();
        for s in inputs {
            let n = s.as_ref().trim().parse::<u64>()?;
            out.push(n);
        }
        Ok(out)
    }
    ```
- If the function has a clear receiver (`self`, `&self`, `&mut self`):
  - Then: implement it as an associated function
  - Else: implement it as a standalone free function
- Add a local `use` statement for enums to minimize the code size. For example:
  - Good:
    ```rust
    pub fn apply(op: GroupsOp) {
        use GroupsOp::*;
        match op {
            InsertOne(_) => {}
            UpdateOne(_, _) => {}
            DeleteOne(_) => {}
        }
    }
    ```
  - Bad:
    ```rust
    pub fn apply(op: GroupsOp) {
        match op {
            GroupsOp::InsertOne(_) => {}
            GroupsOp::UpdateOne(_, _) => {}
            GroupsOp::DeleteOne(_) => {}
        }
    }
    ```
- Simplify the callsite code by accepting `impl Into`. For example:
  - Good:
    ```rust
    pub fn foo(input: impl Into<String>) {
        let input = input.into();
        // do something
    }
    ```
  - Bad:
    ```rust
    /// This is bad because the callsite may have to call .into() when passing the input argument
    pub fn foo(input: String) {}
    ```
- Provide additional flexibility for callsite by accepting `&impl AsRef` or `&mut impl AsMut` (e.g. both `PathBuf` and `Config` may implement `AsRef<Path>`). For example:
  - Good:
    ```rust
    pub fn bar(input: &mut impl AsMut<String>) {
        let input = input.as_mut();
        // do something
    }

    pub fn baz(input: &impl AsRef<str>) {
        let input = input.as_ref();
        // do something
    }
    ```
  - Bad:
    ```rust
    /// This is bad because the callsite may have to call .as_mut() when passing the input argument
    pub fn bar(input: &mut String) {}

    /// This is bad because the callsite may have to call .as_ref() when passing the input argument
    pub fn baz(input: &str) {}
    ```
- Prefer `.map()` instead of `match` when you need to modify the value in the `Option` or `Result`. For example:
  - Good:
    ```rust
    use core::str::FromStr;
    use core::num::ParseIntError;

    impl FromStr for UserId {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            s.parse::<u64>().map(Self::new)
        }
    }
    ```
  - Bad:
  ```rust
  use core::str::FromStr;
  use core::num::ParseIntError;

  impl FromStr for UserId {
      type Err = ParseIntError;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
          // This is bad because it uses more code to express the same idea
          match s.parse::<u64>() {
              Ok(value) => Ok(Self::new(value)),
              Err(error) => Err(error),
          }
      }
  }
  ```
- Use `Self` instead of type name in the `impl` items. For example:
  - Good:
  ```rust
  use core::time::Duration;

  impl From<Duration> for UnixTimestamp {
      #[inline]
      fn from(duration: Duration) -> Self {
          Self::new(duration.as_secs())
      }
  }
  ```
  - Bad:
  ```rust
  use core::time::Duration;

  impl From<Duration> for UnixTimestamp {
      #[inline]
      fn from(duration: Duration) -> Self {
          UnixTimestamp::new(duration.as_secs())
      }
  }
  ```
- Prefer short method syntax over long trait-path syntax (prefer `value.method(arg)` over `Trait::method(value, arg)`)
- Generic helper functions must be in `src/functions` (one file per function)
- If clippy reports `too_many_arguments`:
  - Don't use tuples to silence the lint
  - Consider refactoring the code for better separation of concerns

#### Struct derives

- Derive `new` from `derive_new` crate for types that need `fn new`
- If the struct derives `Getters`, then each field whose type implements `Copy` must have a `#[getter(copy)]` annotation. For example:
  - Good (note that `username` doesn't have `#[getter(copy)]` because its type is `String` which doesn't implement `Copy`, but `age` has `#[getter(copy)]`, because its type is `u64` which implements `Copy`):
    ```rust
    #[derive(Getters, Into, Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
    pub struct User {
      username: String,
      #[getter(copy)]
      age: u64,
    }
    ```

#### Setters

- Use setters that take `&mut self` instead of setters that take `self` and return `Self` (because passing a `foo: &mut Foo` is more efficient than passing `foo: Foo` and returning `Foo` through the call stack)

#### Enums

- When writing code related to enums, bring the variants in scope with `use Enum::*;` statement at the top of the file or function (prefer "at the top of the file" for data enums, prefer "at the top of the function" for error enums).

#### Arithmetic

- Don't use the impls of traits `core::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign, Neg, Shl, ShlAssign, Shr, ShrAssign}` or their operators unless they don't panic or silently overflow
- Write and use arithmetic trait impls that don't panic or silently overflow
- Prefer `checked` versions of arithmetic operations
- Every call to an `overflowing`, `saturating`, `wrapping` version must have a single-line comment above it that starts with "SAFETY: " and describes why calling this version is safe in this specific case
- Use `num` crate items if necessary (for example, to implement a function that calls arithmetic methods on a generic type)

#### Index access

- Never use the following operators: `[], []=`
- Never use the following traits: `core::ops::{Index, IndexMut}`
- If you are sure that `get` or `get_mut` will never panic, use `expect` with a proof message (as described in [Functions](#functions))

Note: the index access operators and traits are banned because they may panic.

#### Test fn

A function marked with `#[test]` or `#[tokio::test]`.

- Must return a `Result`
- Must implement proper error handling via `errgonomic` crate
- Should use macros from `assertables` crate
  - Should use `assert_infix` instead of `assert_gt`, `assert_ge`, `assert_lt`, `assert_le`, `assert_eq`

#### Macros

- Write `macro_rules!` macros to reduce boilerplate
- If you see similar code in different places, write a macro and replace the similar code with a macro call
- If the macros has variadic args:
  - Then: do add `$(,)?`
  - Else: don't add `$(,)?`

#### Shell

- Don't use hard wraps to enforce max line length (I'll use soft wraps in my editor)
- If a command is an argument of a tool call:
  - Then:
    - Prefer short options
  - Else:
    - If it's a common command (one of: set, cd, cp, mv, rm, mkdir, ls, ln, chmod, chown):
      - Then:
        - Prefer short options
      - Else
        - Prefer long options
    - Prefer `echo` instead of `printf`

#### Cargo.toml

- Don't define package features with only a single optional dependency (such features are already defined by cargo automatically)
- Use `cargo add` to add dependencies
- When adding a dependency from crates.io:
  - If the package is [publishable](#publishable-package):
    - Then:
      - Run `cargo add {dependency}@{version}`
        - `{version}` patch component must be 0
      - Try `cargo update -p {dependency} --precise {version}` to lock that exact version
        - If dependency constraints prevent locking that version:
          - Keep the version resolved by Cargo
          - Add a comment in Cargo.toml explaining the constraints
    - Else:
      - Run `cargo add {dependency}` without `{version}`
- When adding a dependency in a workspace:
  - Add it to top-level manifest first (`workspace.dependencies`)
- When adding a dependency for a workspace member:
  - Run `cargo add {dependency}` without `{version}` (cargo will set `workspace = true`)
- When adding a new workspace member: add it to `packages` dir unless specified otherwise

#### Code style

- Don't enforce a line length limit when writing code, comments or documentation

#### Chat thread id

- Must be a string
- Must contain at least 3 characters
- Must contain only uppercase characters

Examples:

- `RVC`
- `AKE`
- `LMY`

Notes:

- Should match the thread topic

#### Chat thread id heading

A Markdown heading level 3 that contains only [chat thread id](#chat-thread-id).

Examples:

- `### RVC`
- `### AKE`
- `### LMY`

#### findings.md

- If it exists:
  - Must contain a non-empty list of [findings](#finding)

#### Finding

- Must be formatted as `### {ctid}\n\n[{priority}] {title}. {body} ({references}). Proposed fixes: {fixes}`
  - `ctid` must be a [chat thread id](#chat-thread-id)
  - `priority` must be one of `P0`, `P1`, `P2`, `P3`.
  - `references` must be a comma-separated list of `reference`
  - `reference` must must be formatted as `{path}:{line}`
  - `path` must be a file path relative to your working directory
  - `line` must be the first line of the relevant code or text block
  - `fixes` must be one of the following:
    - If there is at least one proposed fix:
      - Then: "\n\n" and a Markdown nested list of fixes where each fix must have a format `{number}. {description}` (the numbers should start from 1 for each list of fixes)
      - Else: the exact text "none."

#### Publishable package

A package that has a remote whose name contains `public` or `pre-public` and ends with `template`.

### Guidelines for `serde`

#### Requirements

- Every input data type must derive `Serialize` and `Deserialize`
- Every `Option`-wrapped field must have attributes:
  - `#[serde(skip_serializing_if = "Option::is_none")]`
- Every `OffsetDateTime` field must have attributes:
  - `#[serde(with = "time::serde::rfc3339")]`
- Every `Option<OffsetDateTime>` field must have attributes:
  - `#[serde(with = "time::serde::rfc3339::option")]`

#### Notes

- It is recommended to use `serde_with` to reduce the code size by avoiding custom `Serialize`/`Deserialize` impls

### Project description

#### Concepts

##### Newtype

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
- Must implement validation and preprocessing in `From` / `TryFrom`, not with checker or preprocessor APIs
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
  - If the newtype has a `Deserialize` derive and is [refined](#refined-newtype):
    - Must have `#[serde(try_from = "I")]` (`I` is the inner type)
    - Must not have `#[serde(transparent)]`
    - To serialize identically to `I`:
      - Must derive `SerializeTransparent` instead of `Serialize`
      - Must not have `#[serde(into = "I")]`
  - If the newtype derives both `Serialize` and `Deserialize` without `from`, `try_from`, or `into`:
    - Must have `#[serde(transparent)]`

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

##### SerializeTransparent

Requirements:

- Must error if input is not a struct with exactly one field.
- Must derive `serde::Serialize` by serializing a reference to the sole field.

##### Mutable newtype

A newtype whose inner value can be mutated.

##### Refined newtype

A newtype that enforces invariants at run-time.

Examples:

- [Name](#name)

Counter-examples:

- [DurationSeconds](#durationseconds)
- [Id](#id)

##### Marked newtype

A newtype that contains an additional field for a generic argument `T`

Examples:

- [Id](#id)

Notes:

- A marked newtype can be raw or refined.

#### Examples

##### Name

`struct Name(String)`

Properties:

- [refined](#refined-newtype): true
- [marked](#marked-newtype): false

Purposes:

- Enforces a run-time invariant that the inner string is not empty.

##### DurationSeconds

`struct DurationSeconds(pub u32)`

Properties:

- [refined](#refined-newtype): false
- [marked](#marked-newtype): false

Purposes:

- Enforces a compile-time check that multiplying two values of its type returns `DurationSecondsSquared` (not `DurationSeconds`).

##### Id

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

##### AbsolutePathBuf

`struct AbsolutePathBuf(PathBuf)`

### Project info

### `git remote`

```shell
origin
repoconf-rust-public-lib-template
url-macro
```

## Project files

### mise.toml

```toml
min_version = "2026.7.13"

[settings]
idiomatic_version_file_enable_tools = ["rust"]
task.output = "keep-order"

[tools]
node = "24.15.0"
deno = "1.46.1"
fnox = "1.33.1"
"npm:@commitlint/config-conventional" = "19.6.0"
"npm:@commitlint/cli" = "19.6.0"
"npm:@commitlint/types" = "19.5.0"
"cargo:cargo-insert-docs" = "1.6.0"
"cargo:cargo-hack" = "0.6.33"
"cargo:cargo-nextest" = "0.9.102"
"cargo:cargo-expand" = "1.0.114"
"cargo:taplo-cli" = "0.10.0"
"cargo:rumdl" = "0.1.0"
"cargo:sd" = "1.0.0"

[hooks]
postinstall = { task = "git:install-hooks" }

[tasks."build"]
run = "cargo build --workspace"

[tasks."check"]
depends = ["cargo:validate-config"]
run = [{ tasks = ["lint", "test"] }]

[tasks."test"]
depends = ["test:code", "test:docs"]

[tasks."lint"]
depends = ["lint:name", "lint:configs", "lint:code", "lint:code:style", "lint:docs", "lint:reports"]

[tasks."lint:name"]
run = [{ task = "fix:name", args = ["--check"] }]

[tasks."lint:configs"]
depends = ["lint:configs:cargo", "lint:configs:fnox"]

[tasks."lint:configs:cargo"]
run = [{ task = "fix:cargo", args = ["--check"] }]

[tasks."lint:configs:fnox"]
run = [{ task = "fix:fnox" }]

[tasks."lint:code"]
run = "cargo clippy --locked --workspace --all-targets --all-features -- -D warnings"

[tasks."lint:code:style"]
run = "cargo fmt --all -- --check"

[tasks."lint:docs"]
run = "rumdl check"

[tasks."test:code"]
run = "fnox --profile test exec --replace -- cargo nextest run --locked --workspace --all-features --no-tests warn"

[tasks."test:code:integration"]
# see also: "agent:test:code:integration"
# `--test-threads 1` because integration tests must be run sequentially
run = [{ task = "test:code", args = ["--ignore-default-filter", "--max-fail", "1", "--test-threads", "1", "integration_tests::"] }]

[tasks."test:code:slow"]
# see also: "agent:test:code:slow"
# `--test-threads` is omitted because slow tests may be run in parallel
run = [{ task = "test:code", args = ["--ignore-default-filter", "--max-fail", "1", "slow_tests::"] }]

[tasks."test:docs"]
env = { RUSTDOCFLAGS = "-D warnings" }
run = "cargo test --locked --workspace --doc --all-features --no-fail-fast --quiet"

[tasks."pre-commit"]
alias = "pre-merge-commit"
run = [{ task = "git:validate-commit" }]

# Compatibility for existing clones whose generated post-commit hook still invokes this task. The installer removes that hook during this one-time migration.
[tasks."post-commit"]
hide = true
run = [{ task = "git:install-hooks" }]

[tasks."commit-msg"]
run = 'mise run --output interleave commitlint -- --edit "$@"'

[tasks."fix"]
depends = ["fix:code", "fix:aux"]

[tasks."fix:aux"]
depends = ["fix:configs", "fix:docs", "fix:agents", "fix:readme"]

[tasks."fix:configs"]
depends = ["fix:cargo", "fix:fnox"]

[tasks."fix:code"]
depends = ["fix:name", "fix:code:style"]

[tasks."fix:code:warnings"]
depends = ["fix:cargo"]
# second pass is needed because "cargo clippy --fix" exits with 0 even if some warnings remain
env = { __CARGO_FIX_YOLO = 'yeah' }
run = [
    "cargo clippy --workspace --all-targets --all-features --fix --allow-dirty --allow-staged",
    { task = "lint:code" },
]

[tasks."fix:code:style"]
# Run after `fix:code:warnings` because both tasks modify the same code files.
depends = ["fix:code:warnings"]
run = "cargo fmt --all"

[tasks."fix:docs"]
depends = ["fix:agents", "fix:readme"]
# use `rumdl check --fix` instead of `rumdl fmt` because `rumdl check --fix` exits with 1 if errors remain (since v0.1.0)
run = "rumdl check --fix"

[tasks."fix:agents"]
# "fix:agents" depends on "fix:code" because it reads the code files
depends = ["fix:name", "fix:configs", "fix:code"]
run = [{ task = "gen:agents" }]

[tasks."gen:readme"]
run = "./README.ts"

[tasks."gen:agents"]
run = "./AGENTS.ts"

[tasks."commitlint"]
run = "commitlint --extends \"$(mise where npm:@commitlint/config-conventional)/node_modules/@commitlint/config-conventional/lib/index.js\""

[tasks."agent:docs:list"]
run = "[ -d .agents/docs ] && find .agents/docs -type f -print || true"
output = "interleave"
quiet = true

[tasks."agent:on:stop"]
depends = ["cargo:validate-config"]
run = [{ task = "fix" }, { task = "agent:test" }]

[tasks."agent:test"]
depends = ["agent:test:code", "agent:test:code:integration", "agent:test:code:slow", "test:docs"]

[tasks."agent:test:code"]
# don't include `--fail-fast` because it's better to let the agent see all failures
# reduce output to save tokens
run = [{ task = "test:code", args = ["--cargo-quiet", "--hide-progress-bar", "--status-level", "fail", "--final-status-level", "flaky"] }]

[tasks."agent:test:code:integration"]
# see also: "test:code:integration"
# `--test-threads 1` because integration tests must be run sequentially
run = [{ task = "test:code", args = ["--cargo-quiet", "--hide-progress-bar", "--status-level", "fail", "--final-status-level", "flaky", "--ignore-default-filter", "--max-fail", "1", "--test-threads", "1", "integration_tests::"] }]

[tasks."agent:test:code:slow"]
# see also: "test:code:slow"
# `--test-threads` is omitted because slow tests may be run in parallel
run = [{ task = "test:code", args = ["--cargo-quiet", "--hide-progress-bar", "--status-level", "fail", "--final-status-level", "flaky", "--ignore-default-filter", "--max-fail", "1", "slow_tests::"] }]
```

### fnox.toml

```toml
#:schema https://fnox.jdx.dev/schema.json

if_missing = "error"
env = "exec"

[providers]
keychain = { type = "keychain", service = "subtype" }
pass = { type = "password-store", prefix = "subtype/" }
age = { type = "age", recipients = [
    "age1sf4r4amev2svqr6llwg8hgtz9n7p5qdh7hh0mavcshzfrmgfduksnq3hql",
    "age1605gsnxpe536sprwccyumq74veg0g80u55n8ggems0t8deau6qdsfnq3m3"
] }
```

### Cargo.toml

```toml
[workspace]
members = ["packages/subtype-macros"]
resolver = "3"

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.88.0"
license = "Apache-2.0 OR MIT"
homepage = "https://github.com/DenisGorbachev/subtype"
repository = "https://github.com/DenisGorbachev/subtype"
keywords = ["newtype", "validation"]
categories = ["rust-patterns"]
exclude = [
    ".*",
    "*.local.*",
    "doc/dev",
    "specs",
    "AGENTS.ts",
    "CargoMetadata.ts",
    "README.ts",
    "AGENTS*.md",
    "CLAUDE*.md",
    "deno.lock",
    "deno.json",
    "commitlint.config.mjs",
    "fnox.toml",
    "mise.toml",
    "rumdl.toml",
    "rustfmt.toml",
    ".yolobox"
]

[workspace.metadata.details]
name = "subtype"
title = "Subtype"
readme = { generate = false }

[workspace.lints.rust]
redundant_imports = "deny"
unused_import_braces = "deny"
# unused_qualifications must not be "deny" because our code style has multiple `use Foo::*;`, and some macros (derive_more::Display, strum::Display, strum::EnumString) produce code with full qualifications
# unused_qualifications = "deny"

[workspace.lints.clippy]
absolute_paths = "deny"
arithmetic_side_effects = "deny"

[package]
name = "subtype"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "Better newtypes"
license.workspace = true
homepage.workspace = true
repository.workspace = true
keywords.workspace = true
categories.workspace = true
exclude.workspace = true

[package.metadata.details]
title = "Subtype"

[lints]
workspace = true

[dependencies]
derive_more = { version = "1.0.0", features = ["error"] }
num-traits = { version = "0.2.0", optional = true }
pretty-type-name = "1.0.0"
serde = { version = "1.0.228", optional = true, features = ["derive"] }
standard-traits = { git = "https://github.com/DenisGorbachev/standard-traits" }
subtype-macros = { version = "0.1.0", path = "packages/subtype-macros", optional = true }
time = { version = "0.3.0", optional = true }

[dev-dependencies]
assert_matches = { version = "1.5.0" }
derive_more = { version = "1.0.0", features = ["full"] }
prae = "0.8.4"
rustc-hash = "2.1.0"
thiserror = "2.0.0"
url = "2.5.0"
errgonomic = { git = "https://github.com/DenisGorbachev/errgonomic" }
serde_test = "1.0.177"
serde = { version = "1.0.228", features = ["derive"] }

[features]
macros = ["dep:subtype-macros"]
```

#### packages/subtype-macros/Cargo.toml

```toml
[package]
name = "subtype-macros"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "Derive macros for newtypes"
license.workspace = true
homepage.workspace = true
repository.workspace = true
keywords.workspace = true
categories.workspace = true
exclude.workspace = true

[package.metadata.details]
title = "Subtype macros"

[lib]
proc-macro = true

[lints]
workspace = true

[dependencies]
proc-macro2 = "1.0.0"
quote = "1.0.0"
syn = "3.0.0"

[dev-dependencies]
assertables = "10.1.0"
```

#### packages/subtype-macros/src/lib.rs

```rust
//! Procedural macros for newtypes.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Index, Member, parse_macro_input};

/// Implements `serde::Serialize` by serializing the newtype's borrowed inner field directly.
///
/// Unlike Serde's `into` container attribute, this implementation does not clone or convert the newtype. The derive accepts tuple and named structs containing exactly one field.
#[proc_macro_derive(SerializeTransparent)]
pub fn derive_serialize_transparent(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_serialize_transparent(input).into()
}

fn expand_serialize_transparent(input: DeriveInput) -> TokenStream2 {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = input;
    let (field_type, member) = match data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => {
                let mut fields_iter = fields.named.into_iter();
                match (fields_iter.next(), fields_iter.next()) {
                    (Some(field), None) => match field.ident {
                        Some(field_ident) => (field.ty, Member::Named(field_ident)),
                        None => {
                            return syn::Error::new_spanned(&ident, "SerializeTransparent requires a named or tuple struct containing exactly one field").into_compile_error();
                        }
                    },
                    _ => {
                        return syn::Error::new_spanned(&ident, "SerializeTransparent requires a struct containing exactly one field").into_compile_error();
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let mut fields_iter = fields.unnamed.into_iter();
                match (fields_iter.next(), fields_iter.next()) {
                    (Some(field), None) => (field.ty, Member::Unnamed(Index::from(0))),
                    _ => {
                        return syn::Error::new_spanned(&ident, "SerializeTransparent requires a struct containing exactly one field").into_compile_error();
                    }
                }
            }
            Fields::Unit => {
                return syn::Error::new_spanned(&ident, "SerializeTransparent requires a struct containing exactly one field").into_compile_error();
            }
        },
        Data::Enum(_) => {
            return syn::Error::new_spanned(&ident, "SerializeTransparent does not support enums").into_compile_error();
        }
        Data::Union(_) => {
            return syn::Error::new_spanned(&ident, "SerializeTransparent does not support unions").into_compile_error();
        }
    };
    let where_predicates = generics
        .where_clause
        .as_ref()
        .map(|where_clause| &where_clause.predicates);
    let (impl_generics, type_generics, _) = generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl #impl_generics ::serde::Serialize for #ident #type_generics
        where
            #field_type: ::serde::Serialize,
            #where_predicates
        {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                ::serde::Serialize::serialize(&self.#member, serializer)
            }
        }
    }
}

#[cfg(test)]
mod tests;
```

#### src/lib.rs

```rust
//! This crate provides helper macros for defining newtypes with validation & preprocessors.
//!
//! ## Features
//!
//! * Clone-free transparent serialization for newtypes via `SerializeTransparent` (feature: `macros`)
//! * Validators
//! * Preprocessors
//! * Postprocessors
//! * Visibility specifiers (e.g. `pub`, `pub(crate)`)
//!
//! ## Footguns
//!
//! * It is possible to circumvent the validation by defining `impl From<String> for Username`. This is because an impl can construct `Username` directly without calling `Username::new`. There is no workaround for this, you just need to be careful.
//! * It is possible to circumvent the validation by adding `#[derive(serde::Deserialize)]`. This is because `impl serde::Deserialize` can construct the target type directly without calling the `new` function. There's a workaround: add `#[serde(try_from = "Foo")]` where Foo is the underlying type of this newtype.
//!
//! ## Gotchas
//!
//! * In macro invocations, the generics and trait bounds must be wrapped in square brackets (`[]`) instead of angle brackets (`<>`). This is a limitation of macro_rules. It applies only to the _definitions_ of generics and trait bounds, not to their usage. See examples:
//!   * `newtype!`
//!     * Good: `newtype!(pub struct ProjectDirectoryRef['a](&'a Path));`
//!     * Bad: `newtype!(pub struct ProjectDirectoryRef<'a>(&'a Path));`
//!   * `validate_as_check!`
//!     * Good: `validate_as_check!(impl[V] Validate<V> for Even where [V: IsEven]);`
//!     * Bad: `validate_as_check!(impl<V> Validate<V> for Even where V: IsEven);`
//!
//! ## Constraint types
//!
//! * Constraint types must be empty structs (without fields)
//! * Constraint types must have the following derive attribute: `#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]`
//!   * `Default` is necessary to display the generic `ValidationError` (its `Display` impl requires `Checker: Display`)
//!   * Other traits are necessary to enable the same derives for newtypes
//!
//! ## Error types
//!
//! * Error types must derive `Error` (obviously)
//! * Error types must take ownership of the value that triggered the error, so that the caller would have access to the value in an error handler
//! * Error type fields must be `pub`
//!
//! ## Design notes
//!
//! * `Min<Minimum, Inclusivity>` is used instead of `Min<Minimum, const INCLUSIVITY: bool>` because it provides more informative error messages.
//! * `Min, Max, Equal` use `PartialOrd, PartialEq` instead of `Ord, Eq` because more types implement the Partial traits. Also, if the developer specifies `pub struct MyFloat(f32 | Min<0.5, Exclusive>)`, they don't want `NAN` to pass this validation. This is exactly what `Min` does by delegating to `PartialCmp`, which returns `false` for `NAN`.
//!
//! ## Similar crates
//!
//! * [refinement](https://crates.io/crates/refinement)
//!   * You can't implement foreign traits on a `Refinement` type (because the `Refinement` type comes from the `refinement` crate, which is also foreign to your code)
//!   * You can't sanitize the value in the constructor
//! * [nutype](https://crates.io/crates/nutype)
//!   * You can't return a specific error from the validation predicate
//!   * You have to wait longer due to increased compilation speed
//!   * You can't use autocompletion while writing out the macro call
//! * [synonym](https://crates.io/crates/synonym)
//!   * You can automatically get certain derives based on the underlying type
//!     * You can't get automatic derives for unknown types (for example, if `Address: Copy`, then `pub struct AddressNewtype(Address)` you won't get `impl Copy for AddressNewtype`, you need to explicitly add `#[derive(Copy)]`)
//! * [aliri_braid](https://crates.io/crates/aliri_braid)
//!   * It only supports `String` as the base type
//! * [prae](https://github.com/teenjuna/prae)
//!   * Very similar design
//!   * Wrapper trait imposes too much structure (TryFrom already provides the Error type)
//! * [tightness](https://github.com/PabloMansanet/tightness)
//!   * Last commit on May 30, 2021
//!   * prae improves on tightness
//! * [semval](https://github.com/slowtec/semval)
//! * [validators](https://crates.io/crates/validators)
//!

#![cfg_attr(not(test), deny(unused_crate_dependencies))]

mod checkers;
mod conjurers;
mod errors;
mod macros;
#[cfg(test)]
mod tests;
mod traits;
mod transformers;

pub use checkers::*;
pub use conjurers::*;
pub use errors::*;
pub use traits::*;
pub use transformers::*;

#[cfg(feature = "macros")]
pub use subtype_macros::SerializeTransparent;
```
