use std::marker::PhantomData;

use derive_more::Error;

use crate::traits::check::Check;
use crate::{transform_as_validate, validate_as_check};

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Min<Minimum, const INCLUSIVE: bool> {
    phantom_minimum: PhantomData<Minimum>,
}

impl<Value, Minimum, const INCLUSIVE: bool> Check<Value> for Min<Minimum, INCLUSIVE>
where
    Value: PartialOrd,
    Minimum: Default + Into<Value>,
{
    fn check(value: &Value) -> bool {
        let minimum = Minimum::default().into();
        if INCLUSIVE {
            value >= &minimum
        } else {
            value > &minimum
        }
    }
}

// TODO: Use fmt_derive
#[derive(Error, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct MinError<Value> {
    pub minimum: Value,
}

validate_as_check!(impl[Value, Minimum, const INCLUSIVE: bool] Validate<Value> for Min<Minimum, INCLUSIVE> where [Value: PartialOrd, Minimum: Default + Into<Value>]);

transform_as_validate!(impl[Value, Minimum, const INCLUSIVE: bool] Transform<Value> for Min<Minimum, INCLUSIVE> where [Value: PartialOrd, Minimum: Default + Into<Value>]);

pub const INCLUSIVE: bool = true;
pub const EXCLUSIVE: bool = false;

pub const TOTAL: bool = true;
pub const PARTIAL: bool = false;

#[cfg(test)]
mod tests {
    use crate::containers::u32::U32;

    use super::*;

    #[test]
    fn must_return_correct_values() {
        type M1 = Min<U32<10>, INCLUSIVE>;
        assert!(M1::check(&15));
        assert!(M1::check(&10));
        assert!(!M1::check(&5));

        type M2 = Min<U32<10>, EXCLUSIVE>;
        assert!(M2::check(&15));
        assert!(!M2::check(&10));
        assert!(!M2::check(&5));
    }
}
