use serde::{Deserialize, Serialize};

/// Contains iOS-specific settings within an app specification stored in a
/// `Polyhorn.toml` file.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Spec {
    /// This is the bundle identifier of your app. On iOS, bundle identifiers
    /// are written in reverse DNS notation. For example, if your company's
    /// domain is https://glacyr.com and the app name is Babel, the bundle
    /// identifier should be `com.glacyr.Babel`.
    #[serde(rename = "bundle-identifier")]
    pub bundle_identifier: String,
}
