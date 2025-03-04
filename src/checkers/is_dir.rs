use crate::Check;
use crate::{transform_as_validate_as_check, validate_as_check};
use std::path::{Path, PathBuf};

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct IsDir;

impl Check<PathBuf> for IsDir {
    fn check(value: &PathBuf) -> bool {
        value.is_dir()
    }
}

transform_as_validate_as_check!(impl of [PathBuf] for IsDir);

impl Check<Path> for IsDir {
    fn check(value: &Path) -> bool {
        value.is_dir()
    }
}

validate_as_check!(impl Validate<Path> for IsDir);
// TODO: can't implement TryTransform on a slice type because it can only be used behind a reference, but TryTransform expects an owned type without a reference
// try_transform_as_check!(impl['a] TryTransform<&'a Path> for IsDir);
