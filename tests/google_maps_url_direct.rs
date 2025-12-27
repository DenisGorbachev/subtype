use derive_more::{AsRef, Deref, Into};
use subtype::impl_try_from_ref_via_owned;
use thiserror::Error;
use url::Url;

#[derive(Deref, AsRef, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct GoogleMapsUrl(Url);

#[derive(Error, Debug, Eq, PartialEq)]
pub enum GoogleMapsUrlFromUrlError {
    #[error("url is missing a host")]
    MissingHost,
    #[error("url host is not a Google Maps host: {host}")]
    InvalidHost { host: String },
    #[error("google.com urls must start with /maps, got: {path}")]
    InvalidPath { path: String },
}

impl TryFrom<Url> for GoogleMapsUrl {
    type Error = GoogleMapsUrlFromUrlError;

    fn try_from(url: Url) -> Result<Self, Self::Error> {
        use GoogleMapsUrlFromUrlError::*;
        let host = url.host_str().ok_or(MissingHost)?;
        let is_shortlink = host == "maps.app.goo.gl";
        let is_google_host = host == "google.com" || host.ends_with(".google.com");
        let path_starts_with_maps = url
            .path_segments()
            .and_then(|mut segments| segments.next().map(|segment| segment == "maps"))
            .unwrap_or(false);
        if is_shortlink {
            return Ok(Self(url));
        }
        if !is_google_host {
            return Err(InvalidHost {
                host: host.to_string(),
            });
        }
        if !path_starts_with_maps {
            return Err(InvalidPath {
                path: url.path().to_string(),
            });
        }
        Ok(Self(url))
    }
}

impl_try_from_ref_via_owned!(impl TryFrom<&Url> for GoogleMapsUrl, Url);

impl TryFrom<String> for GoogleMapsUrl {
    type Error = GoogleMapsUrlFromStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let url = Url::parse(&value)?;
        Ok(GoogleMapsUrl::try_from(url)?)
    }
}

impl_try_from_ref_via_owned!(impl TryFrom<&str> for GoogleMapsUrl, String);

#[derive(Error, Debug)]
pub enum GoogleMapsUrlFromStringError {
    #[error("failed to parse url")]
    Parse {
        #[from]
        source: url::ParseError,
    },
    #[error("invalid google maps url")]
    InvalidUrl {
        #[from]
        source: GoogleMapsUrlFromUrlError,
    },
}

#[test]
fn must_try_from_url() {
    assert!(GoogleMapsUrl::try_from("https://maps.app.goo.gl/v42d7shTot2QmUcF9").is_ok());
    assert!(GoogleMapsUrl::try_from("https://www.google.com/maps/place/Air+Force+Memorial/@38.8668639,-77.0745191,14.84z/data=!4m15!1m8!3m7!1s0x808fb9fe5f285e3d:0x8b5109a227086f55!2sCalifornia!3b1!8m2!3d36.778261!4d-119.4179324!16zL20vMDFuN3E!3m5!1s0x89b7b6dd6055b143:0x55f6b73e6f5bbb96!8m2!3d38.8684018!4d-77.0664438!16zL20vMDlkZ216?entry=ttu&g_ep=EgoyMDI1MTIwOS4wIKXMDSoKLDEwMDc5MjA3M0gBUAM%3D").is_ok());
}
