use derive_more::{AsRef, Deref, Into};
use subtype::impl_try_from_ref_via_owned;
use url::Url;

#[derive(Deref, AsRef, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct GoogleMapsUrl(Url);

impl TryFrom<Url> for GoogleMapsUrl {
    type Error = ();

    fn try_from(_url: Url) -> Result<Self, Self::Error> {
        // TODO: validate the url
        // TODO: pass if domain == maps.app.goo.gl
        // TODO: pass if domain ends with google.com and path starts with maps
        todo!()
    }
}

impl_try_from_ref_via_owned!(impl TryFrom<&Url> for GoogleMapsUrl, Url);

impl TryFrom<String> for GoogleMapsUrl {
    type Error = ();

    fn try_from(_value: String) -> Result<Self, Self::Error> {
        // TODO: parse url, then call Self::try_from
        todo!()
    }
}

impl_try_from_ref_via_owned!(impl TryFrom<&str> for GoogleMapsUrl, String);

#[ignore]
#[test]
fn must_try_from_url() {
    assert!(GoogleMapsUrl::try_from("https://maps.app.goo.gl/v42d7shTot2QmUcF9").is_ok());
    assert!(GoogleMapsUrl::try_from("https://www.google.com/maps/place/Air+Force+Memorial/@38.8668639,-77.0745191,14.84z/data=!4m15!1m8!3m7!1s0x808fb9fe5f285e3d:0x8b5109a227086f55!2sCalifornia!3b1!8m2!3d36.778261!4d-119.4179324!16zL20vMDFuN3E!3m5!1s0x89b7b6dd6055b143:0x55f6b73e6f5bbb96!8m2!3d38.8684018!4d-77.0664438!16zL20vMDlkZ216?entry=ttu&g_ep=EgoyMDI1MTIwOS4wIKXMDSoKLDEwMDc5MjA3M0gBUAM%3D").is_ok());
}
