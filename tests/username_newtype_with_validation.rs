use subtype::constraints::non_empty::NonEmpty;
use subtype::newtype;
use subtype::validation_error::ValidationError;

newtype!(
    #[derive(PartialOrd, PartialEq, Clone, Debug)]
    pub struct UsernameTupleNonEmpty(String | NonEmpty);
);

newtype!(
    #[derive(PartialOrd, PartialEq, Clone, Debug)]
    pub struct UsernameRegularNonEmpty {
        inner: String | NonEmpty
    }
);

newtype!(
    #[derive(PartialOrd, PartialEq, Clone, Debug)]
    pub struct UsernameTupleAny(String);
);

newtype!(
    #[derive(PartialOrd, PartialEq, Clone, Debug)]
    pub struct UsernameRegularAny {
        inner: String,
    }
);

fn main() {
    assert_eq!(UsernameTupleNonEmpty::new(""), Err(ValidationError::<NonEmpty>::new()));
    assert_eq!(UsernameTupleNonEmpty::new("alice"), Ok(UsernameTupleNonEmpty("alice".to_string())));
    assert_eq!(UsernameRegularNonEmpty::new(""), Err(ValidationError::<NonEmpty>::new()));
    assert_eq!(
        UsernameRegularNonEmpty::new("alice"),
        Ok(UsernameRegularNonEmpty {
            inner: "alice".to_string()
        })
    );
    assert_eq!(UsernameTupleAny::new(""), UsernameTupleAny("".to_string()));
    assert_eq!(UsernameTupleAny::new("alice"), UsernameTupleAny("alice".to_string()));
    assert_eq!(
        UsernameRegularAny::new(""),
        UsernameRegularAny {
            inner: "alice".to_string()
        }
    );
    assert_eq!(
        UsernameRegularAny::new("alice"),
        UsernameRegularAny {
            inner: "alice".to_string()
        }
    );
}
