use serde::{Deserialize, Serialize};

/// This spec contains Android-specific settings.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Spec {
    /// This is the package identifier for Android that uniquely identifies your
    /// app on a user's device and within the Google Play Store. The package
    /// identifier usually looks like a reverse DNS name. The package identifier
    /// must contain of at least two segments, each segment must start with a
    /// letter and can only contain alphanumeric characters.
    pub package: String,

    /// This is the version code for Android. The version code is used to
    /// determine the order of multiple releases of the same app. Each
    /// subsequent release must have a version code that is strictly greater
    /// than the previous release's version code (but it is allowed to skip
    /// version codes in between). Additionally, this is also used by the OS to
    /// deny downgrades to earlier versions of your app (e.g. to protect against
    /// cases where you introduce backwards incompatible changes to a persistent
    /// storage format).
    #[serde(rename = "version-code")]
    pub version_code: usize,

    /// This is the minimum API version that your app supports. If you're not
    /// bringing in your own native Java code, this should generally not be
    /// changed. In any case, don't change this to an API version lower than the
    /// Polyhorn default (16), because our own Java code might not be compatible
    /// with earlier API versions.
    #[serde(rename = "min-sdk-version", default = "Spec::default_min_sdk_version")]
    pub min_sdk_version: usize,

    /// This is the target API version that your app uses to build. We're
    /// currently compiling native Java code against the most recent Android API
    /// level. If you don't bring in your own Java code, you shouldn't need to
    /// change this. Even if you do bring in your own Java code, it's unlikely
    /// that you need to change this.
    #[serde(
        rename = "target-sdk-version",
        default = "Spec::default_target_sdk_version"
    )]
    pub target_sdk_version: usize,

    /// This is the name of the shared library that will contain your Rust code.
    /// End users will not be aware of this. It simply refers to the name of the
    /// shared library that will reside in `jniLibs/*`. Usually, this will be
    /// the lowercased last segment of the package identifier.
    pub library: String,
}

impl Spec {
    fn default_min_sdk_version() -> usize {
        16
    }

    fn default_target_sdk_version() -> usize {
        30
    }
}
