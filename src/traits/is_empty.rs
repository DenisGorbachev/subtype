pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for str {
    fn is_empty(&self) -> bool {
        str::is_empty(self)
    }
}

impl IsEmpty for &str {
    fn is_empty(&self) -> bool {
        str::is_empty(self)
    }
}

impl IsEmpty for String {
    fn is_empty(&self) -> bool {
        String::is_empty(self)
    }
}

impl IsEmpty for &String {
    fn is_empty(&self) -> bool {
        String::is_empty(self)
    }
}

impl<T> IsEmpty for [T] {
    fn is_empty(&self) -> bool {
        <[T]>::is_empty(self)
    }
}

impl<T> IsEmpty for &[T] {
    fn is_empty(&self) -> bool {
        <[T]>::is_empty(self)
    }
}

impl<T> IsEmpty for Vec<T> {
    fn is_empty(&self) -> bool {
        <Vec<T>>::is_empty(self)
    }
}

impl<T> IsEmpty for &Vec<T> {
    fn is_empty(&self) -> bool {
        <Vec<T>>::is_empty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_provide_impl_for_vec() {
        type T = Vec<i32>;
        let value: T = vec![];
        assert!(<T as IsEmpty>::is_empty(&value))
    }
}
