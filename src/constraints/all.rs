use std::marker::PhantomData;

use crate::traits::check::Check;

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct All<Validator>(PhantomData<Validator>);

impl<Item, Value, Checker> Check<Value> for All<Checker>
where
    for<'a> &'a Value: IntoIterator<Item = &'a Item>,
    Checker: Check<Item>,
{
    fn check(value: &Value) -> bool {
        value.into_iter().all(|item| Checker::check(item))
    }
}
