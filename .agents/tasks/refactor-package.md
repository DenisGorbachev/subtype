# Refactor the whole package

## Rationale

It's better to write an explicit `impl TryFrom` for preprocessing, checking, postprocessing the inner value.

## Tasks

- Refactor all macros
  - Remove the arms with `$checker`
  - Remove `$preprocessor` and `$postprocessor` from remaining arms
- Refactor `subtype!` macro
- Remove the following macros and their invocations:
  - `purify_with_generic_validation!`
  - `purify_without_validation!`
- Remove the following mods and their child mods:
  - `checkers`
  - `conjurers`
  - `transformers`
  - `errors`
- Remove unused traits
