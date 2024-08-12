#[macro_export]
macro_rules! with_derive_neg {
    (
        $input:tt
    ) => {
        #[derive_more::Neg]
        $input
    };
}
