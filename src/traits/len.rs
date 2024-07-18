pub trait Len {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Len for String {
    /// Note that it calculates the length of string in characters, not bytes
    fn len(&self) -> usize {
        self.chars().count()
    }

    fn is_empty(&self) -> bool {
        String::is_empty(self)
    }
}

impl<T> Len for Vec<T> {
    fn len(&self) -> usize {
        Vec::<T>::len(self)
    }

    fn is_empty(&self) -> bool {
        Vec::<T>::is_empty(self)
    }
}
