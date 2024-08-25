use crate::Inclusive;
use crate::InvalidValueError;
use crate::Validate;
use crate::U32;
use crate::{Empty, Max};

#[cfg(test)]
mod validate {
    use crate::LessThanOrEqual;

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

#[cfg(test)]
mod tuples {
    use crate::MaxLen;
    use crate::Not;
    use crate::{IncorrectValueError, ValidationError, ValidationError2};

    use super::*;

    #[test]
    fn must_display_good_error_message() {
        type TheTupleError = ValidationError2<ValidationError<Not<Empty>>, ValidationError<MaxLen<255, Inclusive>>>;
        type TheError = IncorrectValueError<String, TheTupleError>;
        let error = TheError::new("hello", TheTupleError::Variant1(ValidationError::new()));
        assert_eq!(error.to_string(), "IncorrectValueError { value: \"hello\", error: Variant1(ValidationError(PhantomData<subtype::checkers::not::Not<subtype::checkers::empty::Empty>>)) }");
    }
}
