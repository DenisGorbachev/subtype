use derive_more::{AsRef, Deref, Into};
use errgonomic::{handle, handle_bool, handle_opt};
use subtype::impl_try_from_ref_via_owned;
use thiserror::Error;
use url::Url;

#[derive(Deref, AsRef, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct GoogleMapsUrl(Url);

impl TryFrom<Url> for GoogleMapsUrl {
    type Error = ConvertUrlToGoogleMapsUrlError;

    fn try_from(url: Url) -> Result<Self, Self::Error> {
        use ConvertUrlToGoogleMapsUrlError::*;
        let host = handle_opt!(url.host_str(), HostNotFound, url);
        let is_shortlink = host == "maps.app.goo.gl";
        let is_google_host = host == "google.com" || host.ends_with(".google.com");
        let path_starts_with_maps = url
            .path_segments()
            .and_then(|mut segments| segments.next().map(|segment| segment == "maps"))
            .unwrap_or(false);
        if is_shortlink {
            return Ok(Self(url));
        }
        handle_bool!(!is_google_host, HostInvalid, host: host.to_string());
        handle_bool!(!path_starts_with_maps, PathInvalid, path: url.path().to_string());
        Ok(Self(url))
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum ConvertUrlToGoogleMapsUrlError {
    #[error("url is missing a host")]
    HostNotFound { url: Url },
    #[error("url host is not a Google Maps host: '{host}'")]
    HostInvalid { host: String },
    #[error("google.com urls must start with /maps, got: '{path}'")]
    PathInvalid { path: String },
}

impl_try_from_ref_via_owned!(impl TryFrom<&Url> for GoogleMapsUrl, Url);

impl TryFrom<String> for GoogleMapsUrl {
    type Error = ConvertStringToGoogleMapsUrlError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        use ConvertStringToGoogleMapsUrlError::*;
        let url = handle!(Url::parse(&value), ParseFailed, value);
        let url = handle!(GoogleMapsUrl::try_from(&url), TryFromFailed, url);
        Ok(url)
    }
}

impl_try_from_ref_via_owned!(impl TryFrom<&str> for GoogleMapsUrl, String);

#[derive(Error, Debug)]
pub enum ConvertStringToGoogleMapsUrlError {
    #[error("failed to parse url from string '{value}'")]
    ParseFailed { source: url::ParseError, value: String },
    #[error("failed to convert url to a Google Maps url")]
    TryFromFailed { source: ConvertUrlToGoogleMapsUrlError, url: Url },
}

#[test]
fn must_try_from_url() {
    assert!(GoogleMapsUrl::try_from("https://maps.app.goo.gl/v42d7shTot2QmUcF9").is_ok());
    assert!(GoogleMapsUrl::try_from("https://www.google.com/maps/place/Air+Force+Memorial/@38.8668639,-77.0745191,14.84z/data=!4m15!1m8!3m7!1s0x808fb9fe5f285e3d:0x8b5109a227086f55!2sCalifornia!3b1!8m2!3d36.778261!4d-119.4179324!16zL20vMDFuN3E!3m5!1s0x89b7b6dd6055b143:0x55f6b73e6f5bbb96!8m2!3d38.8684018!4d-77.0664438!16zL20vMDlkZ216?entry=ttu&g_ep=EgoyMDI1MTIwOS4wIKXMDSoKLDEwMDc5MjA3M0gBUAM%3D").is_ok());
}
