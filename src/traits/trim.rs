pub trait Trim {
    fn trim(&mut self);
}

impl Trim for String {
    // TODO: Write a more efficient implementation with String::truncate and String::drain
    fn trim(&mut self) {
        *self = str::trim(self.as_str()).to_string()
    }
}

// #[cfg(feature = "num-traits")]
// impl<T: num_traits::Zero> Trim for Vec<T> {
//     // TODO: Write a more efficient implementation
//     fn trim(self) -> Self {
//         todo!()
//     }
// }
