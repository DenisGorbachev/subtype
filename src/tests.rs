use crate::checkers::{Empty, Max};
use crate::containers::inclusive::Inclusive;
use crate::containers::u32::U32;
use crate::errors::invalid_value_error::InvalidValueError;
use crate::traits::validate::Validate;

#[cfg(test)]
mod validate {
    use crate::checkers::LessThanOrEqual;

    use super::*;

    #[test]
    fn must_display_informative_error_message() {
        let value = "hello".to_string();
        let error = Empty::validate(&value).unwrap();
        assert_eq!(error.to_string(), "ValidationError { validator: \"Empty\" }")
    }

    #[test]
    fn must_display_informative_error_message_for_generic_types() {
        let value = 10u32;
        let error = LessThanOrEqual::<U32<5>>::validate(&value).unwrap();
        assert_eq!(error.to_string(), "ValidationError { validator: \"Max<U32<5>, Inclusive>\" }")
    }
}

#[cfg(test)]
mod transform {
    use super::*;

    #[test]
    fn must_display_good_error_message() {
        let error = InvalidValueError::<String, Max<U32<20>, Inclusive>>::new("hello");
        assert_eq!(error.to_string(), "InvalidValueError { value: \"hello\", validator: \"Max<U32<20>, Inclusive>\" }");
    }
}
