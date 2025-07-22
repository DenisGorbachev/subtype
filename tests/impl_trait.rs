use subtype::{subtype_string, IsEmpty, Not, Trim};

subtype_string!(
    pub struct UsernameNotEmpty(String [Trim] | Not<IsEmpty>);
);

#[test]
fn must_implement_traits() {
    let alice = UsernameNotEmpty::try_from(" alice ").unwrap();
    assert_eq!(alice, "alice");
    assert_ne!(alice, "bob");
}
