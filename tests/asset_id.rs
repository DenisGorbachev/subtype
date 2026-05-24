use subtype::Space;
use subtype::subtype_string;
use subtype::{Contains, IsEmpty, Not};
use subtype::{IncorrectValueError, ValidationError, ValidationError2};

subtype_string!(
    pub struct AssetId(String | (Not<IsEmpty>, Not<Contains<Space>>))
);

#[test]
fn asset_id() {
    type InnerError = ValidationError2<ValidationError<Not<IsEmpty>>, ValidationError<Not<Contains<Space>>>>;
    type TheError = IncorrectValueError<String, InnerError>;
    assert_eq!(AssetId::new(""), Err(TheError::new("", InnerError::Variant1(ValidationError::<Not<IsEmpty>>::new()))));
    assert_eq!(AssetId::new("usd"), Ok(AssetId("usd".to_string())));
}
