use subtype::constraints::non_empty::NonEmpty;
use subtype::errors::InvalidValueError;
use subtype::newtype;

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

#[test]
fn username_newtype_with_validation() {
    assert_eq!(UsernameTupleNonEmpty::new(""), Err(InvalidValueError::<String, NonEmpty>::new("")));
    assert_eq!(UsernameTupleNonEmpty::new("alice"), Ok(UsernameTupleNonEmpty("alice".to_string())));
    assert_eq!(UsernameRegularNonEmpty::new(""), Err(InvalidValueError::<String, NonEmpty>::new("")));
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
            inner: "".to_string()
        }
    );
    assert_eq!(
        UsernameRegularAny::new("alice"),
        UsernameRegularAny {
            inner: "alice".to_string()
        }
    );
}
