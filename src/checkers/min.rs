use std::marker::PhantomData;

use crate::transform_as_validate_as_check;
use crate::Check;
use crate::Conjure;
use crate::Exclusive;
use crate::Inclusive;

// This order and meta-type of generic arguments is better for informative errors
#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Min<Minimum, Inclusivity>(PhantomData<(Minimum, Inclusivity)>);

pub type GreaterThan<Minimum> = Min<Minimum, Exclusive>;
pub type GreaterThanOrEqual<Minimum> = Min<Minimum, Inclusive>;

impl<Value, Minimum, Inclusivity> Check<Value> for Min<Minimum, Inclusivity>
where
    Value: PartialOrd,
    Minimum: Conjure<Value>,
    Inclusivity: Conjure<bool>,
{
    fn check(value: &Value) -> bool {
        let minimum = Minimum::conjure();
        if Inclusivity::conjure() {
            value >= &minimum
        } else {
            value > &minimum
        }
    }
}

transform_as_validate_as_check!(impl[Value, Minimum, Inclusivity] of [Value] for Min<Minimum, Inclusivity> where [Value: PartialOrd, Minimum: Conjure<Value>, Inclusivity: Conjure<bool>]);

#[cfg(test)]
mod tests {
    use crate::Exclusive;
    use crate::Inclusive;
    use crate::U32;

    use super::*;

    #[test]
    fn must_return_correct_values() {
        type M1 = Min<U32<10>, Inclusive>;
        assert!(M1::check(&15u32));
        assert!(M1::check(&10u32));
        assert!(!M1::check(&5u32));

        type M2 = Min<U32<10>, Exclusive>;
        assert!(M2::check(&15u32));
        assert!(!M2::check(&10u32));
        assert!(!M2::check(&5u32));
    }
}
