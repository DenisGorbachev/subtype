use std::marker::PhantomData;

use crate::traits::check::Check;
use crate::traits::conjure::Conjure;
use crate::{transform_as_validate, validate_as_check};

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Min<Minimum, const INCLUSIVE: bool> {
    phantom_minimum: PhantomData<Minimum>,
}

impl<Value, Minimum, const INCLUSIVE: bool> Check<Value> for Min<Minimum, INCLUSIVE>
where
    Value: PartialOrd,
    Minimum: Conjure<Value>,
{
    fn check(value: &Value) -> bool {
        let minimum = Minimum::conjure();
        if INCLUSIVE {
            value >= &minimum
        } else {
            value > &minimum
        }
    }
}

validate_as_check!(impl[Value, Minimum, const INCLUSIVE: bool] Validate<Value> for Min<Minimum, INCLUSIVE> where [Value: PartialOrd, Minimum: Conjure<Value>]);

transform_as_validate!(impl[Value, Minimum, const INCLUSIVE: bool] Transform<Value> for Min<Minimum, INCLUSIVE> where [Value: PartialOrd, Minimum: Conjure<Value>]);

// TODO: Move to a separate crate & module
pub const INCLUSIVE: bool = true;
pub const EXCLUSIVE: bool = false;

#[cfg(test)]
mod tests {
    use crate::containers::u32::U32;

    use super::*;

    #[test]
    fn must_return_correct_values() {
        type M1 = Min<U32<10>, INCLUSIVE>;
        assert!(M1::check(&15u32));
        assert!(M1::check(&10u32));
        assert!(!M1::check(&5u32));

        type M2 = Min<U32<10>, EXCLUSIVE>;
        assert!(M2::check(&15u32));
        assert!(!M2::check(&10u32));
        assert!(!M2::check(&5u32));
    }
}
