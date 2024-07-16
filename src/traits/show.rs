/// `Show` is like `Display` but for types.
///
/// `Show` converts a type to a string. This is different from the `Display` trait because it works without a specific value (doesn't take a `self` parameter).
pub trait Show {
    fn show() -> String;
}
