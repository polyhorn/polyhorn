//! Types and functions that implement the specification of `Polyhorn.toml`
//! files.

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod error;

pub use error::Error;

use crate::{android, ios};

/// Specification stored in a `Polyhorn.toml` file.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Spec {
    /// Settings related to the app that a Polyhorn project builds.
    pub app: AppSpec,
}

impl Spec {
    /// Attempts to open the specification stored in the `Polyhorn.toml` file at
    /// the given path.
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

/// Settings related to the app that a Polyhorn project builds.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AppSpec {
    /// This is the name that will appear on the home screen.
    pub name: String,

    /// This is the human-readable version string. The version string is not
    /// bound to any of the constraints that the version code (for Android) is
    /// bound to. Specifically: this string can consist of any characters, it is
    /// not used for determining an ordering between releases and therefore does
    /// not have to be incrementing.
    pub version: String,

    /// These are iOS-specific settings.
    pub ios: ios::Spec,

    /// These are Android-specific settings.
    pub android: android::Spec,
}
