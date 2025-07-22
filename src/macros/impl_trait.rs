#[macro_export]
macro_rules! impl_binop_trait {
    (impl $trt:ident<$other:ty> for $typ:ty; method = $method:ident; output = $output:ty) => {
        impl $trt<$other> for $typ {
            fn $method(&self, other: &$other) -> $output {
                self.0.$method(other)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_partial_eq {
    ($typ:ty, $other:ty) => {
        $crate::impl_binop_trait!(impl PartialEq<$other> for $typ; method = eq; output = bool);
    };
}

// TODO: implement it (may require calling `self.0.as_ref()`)
// #[macro_export]
// macro_rules! impl_partial_ord {
//     ($typ:ty, $other:ty) => {
//         $crate::impl_binop_trait!(impl PartialOrd<$other> for $typ; method = partial_cmp; output = Option<std::cmp::Ordering>);
//     };
// }
