use std::fmt::Debug;
use subtype::constraints::pass::Pass;
use subtype::impl_new;

pub struct Generic<T: Iterator>(T);

impl_new!(Pass, T, impl<T> for Generic<T> where T: Iterator, T: Debug);
