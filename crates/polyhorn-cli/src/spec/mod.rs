use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Spec {
    pub app: AppSpec,
}

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    TOML(toml::de::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Error::TOML(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {}

impl Spec {
    pub fn open<P>(path: P) -> Result<Spec, Error>
    where
        P: AsRef<Path>,
    {
        let mut file = File::open(path)?;
        let mut bytes = vec![];
        file.read_to_end(&mut bytes)?;
        Ok(toml::from_slice::<Spec>(&bytes)?)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct AppSpec {
    /// This is the name that will appear on the home screen.
    pub name: String,

    /// These are iOS-specific settings.
    pub ios: IOSSpec,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct IOSSpec {
    /// This is the bundle identifier of your app. On iOS, bundle identifiers are
    /// written in reverse DNS notation. For example, if your company's domain is
    /// https://glacyr.com and the app name is Babel, the bundle identifier
    /// should be `com.glacyr.Babel`.
    #[serde(rename = "bundle-identifier")]
    pub bundle_identifier: String,
    // pub uses_non_exempt_encryption: bool,
}
