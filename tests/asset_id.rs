use subtype::newtype_string;
use subtype::Space;
use subtype::{Contains, Empty, Not};
use subtype::{IncorrectValueError, ValidationError, ValidationError2};

newtype_string!(
    pub struct AssetId(String | (Not<Empty>, Not<Contains<Space>>))
);

#[test]
fn asset_id() {
    type InnerError = ValidationError2<ValidationError<Not<Empty>>, ValidationError<Not<Contains<Space>>>>;
    type TheError = IncorrectValueError<String, InnerError>;
    assert_eq!(AssetId::new(""), Err(TheError::new("", InnerError::Variant1(ValidationError::<Not<Empty>>::new()))));
    assert_eq!(AssetId::new("usd"), Ok(AssetId("usd".to_string())));
}
