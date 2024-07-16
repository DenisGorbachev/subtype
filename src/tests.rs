use crate::constraints::{Empty, Min};
use crate::containers::inclusive::Inclusive;
use crate::containers::u32::U32;
use crate::traits::validate::Validate;

#[test]
fn must_display_correct_error_message() {
    let value = "hello".to_string();
    let error = Empty::validate(&value).unwrap();
    assert_eq!(error.to_string(), "validation error: Empty")
}

#[test]
fn must_display_correct_error_message_for_generic_types() {
    let value = 10u32;
    let error = Min::<U32<20>, Inclusive>::validate(&value).unwrap();
    assert_eq!(error.to_string(), "validation error: Min<U32<20>, Inclusive>")
}
