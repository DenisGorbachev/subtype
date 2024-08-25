use std::marker::PhantomData;

use crate::conjurers::exclusive::Exclusive;
use crate::conjurers::inclusive::Inclusive;
use crate::transform_as_validate_as_check;
use crate::Check;
use crate::Conjure;

// This order and meta-type of generic arguments is better for informative errors
#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Max<Maximum, Inclusivity>(PhantomData<(Maximum, Inclusivity)>);

pub type LessThan<Maximum> = Max<Maximum, Exclusive>;
pub type LessThanOrEqual<Maximum> = Max<Maximum, Inclusive>;

impl<Value, Maximum, Inclusivity> Check<Value> for Max<Maximum, Inclusivity>
where
    Value: PartialOrd,
    Maximum: Conjure<Value>,
    Inclusivity: Conjure<bool>,
{
    fn check(value: &Value) -> bool {
        let maximum = Maximum::conjure();
        if Inclusivity::conjure() {
            value <= &maximum
        } else {
            value < &maximum
        }
    }
}

transform_as_validate_as_check!(impl[Value, Maximum, Inclusivity] of [Value] for Max<Maximum, Inclusivity> where [Value: PartialOrd, Maximum: Conjure<Value>, Inclusivity: Conjure<bool>]);

#[cfg(test)]
mod tests {
    use crate::conjurers::exclusive::Exclusive;
    use crate::conjurers::inclusive::Inclusive;
    use crate::conjurers::u32::U32;

    use super::*;

    #[test]
    fn must_return_correct_values() {
        type M1 = Max<U32<10>, Inclusive>;
        assert!(!M1::check(&15u32));
        assert!(M1::check(&10u32));
        assert!(M1::check(&5u32));

        type M2 = Max<U32<10>, Exclusive>;
        assert!(!M2::check(&15u32));
        assert!(!M2::check(&10u32));
        assert!(M2::check(&5u32));
    }
}
