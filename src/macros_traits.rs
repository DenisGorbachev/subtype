#[macro_export]
macro_rules! transform_as_validate_as_check {
    (impl$([$($generics:tt)*])? of [$target:ty] for $checker:ty $(where [$($where_clause:tt)*])?) => {
        $crate::validate_as_check!(impl$([$($generics)*])? Validate<$target> for $checker $(where [$($where_clause)*])?);
        $crate::try_transform_as_check!(impl$([$($generics)*])? Transform<$target> for $checker $(where [$($where_clause)*])?);
    };
}
