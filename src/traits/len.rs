pub trait Len {
    type Size;

    fn len(&self) -> Self::Size;

    /// A default implementation can't be provided because `Self::Size` may not implement `PartialEq` and `Zero`
    fn is_empty(&self) -> bool;
}
