// TODO: Extract into a separate trait

pub trait Len {
    type Size;

    fn len() -> Self::Size;
}
