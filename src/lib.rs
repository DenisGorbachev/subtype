//! # Similar crates
//!
//! * [refinement](https://docs.rs/refinement/latest/refinement/)
//!   * You can't foreign traits on a `Refinement` type (because the `Refinement` type comes from the `refinement` crate, which is also foreign to your code)
//!   * You can't sanitize the value in the constructor
//! * [nutype](https://github.com/greyblake/nutype)
//!   * You can't return a specific error from the validation predicate
//!   * You have to wait longer due to increased compilation speed
//!   * You can't use autocompletion while writing out the macro call

pub mod constraints;
pub mod subtype;
pub mod traits;
pub mod validation_error;
