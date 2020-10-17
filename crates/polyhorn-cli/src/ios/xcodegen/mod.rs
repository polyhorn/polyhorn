//! Rust-wrapper around the third-party `xcodegen` utility (installed through
//! Homebrew) that makes it possible to generate Xcode project files
//! programmatically. Note: the documentation within this module is taken
//! directly from the original excellent documentation of `xcodegen`.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Contains the `xcodegen` specification of a project.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Project {
    /// Name of the generated project.
    pub name: String,

    /// The list of targets in the project mapped by name.
    pub targets: HashMap<String, Target>,
}

/// Represents a target that is built within a project.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Target {
    /// Product type of the target.
    #[serde(rename = "type")]
    pub product_type: ProductType,

    /// Platform of the target.
    pub platform: HashSet<Platform>,

    /// The deployment target (e.g. `9.2`). If this is not specified the value
    /// from the project set in `Options.deploymentTarget.PLATFORM` will be used.
    #[serde(rename = "deploymentTarget")]
    pub deployment_targets: HashMap<Platform, String>,

    /// Source directories of the target.
    pub sources: Vec<TargetSource>,

    /// Target specific build settings. Default platform and product type
    /// settings will be applied first before any custom settings defined here.
    /// Other context dependant settings will be set automatically as well.
    pub settings: HashMap<String, String>,

    /// Dependencies for the target.
    pub dependencies: Vec<Dependency>,
}

/// This will provide default build settings for a certain product type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ProductType {
    /// Represents a regular application.
    #[serde(rename = "application")]
    Application,

    /// Represents an App Clip.
    #[serde(rename = "application-on-demand-install-capable")]
    ApplicationOnDemandInstallCapable,
}

/// Indicates the platform that a target's product can run on.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Platform {
    /// Represents an application or extension that runs on iOS.
    #[serde(rename = "iOS")]
    IOS,

    /// Represents an application or extension that runs on macOS.
    #[serde(rename = "macOS")]
    MacOS,

    /// Represents an application or extension that runs on tvOS.
    #[serde(rename = "tvOS")]
    TVOS,

    /// Represents an application or extension that runs on watchOS.
    #[serde(rename = "watchOS")]
    WatchOS,
}

/// Represents a source location that is included for a particular target within
/// an Xcode project.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TargetSource {
    /// The path of the source file or directory.
    pub path: String,
}

/// Represents a dependency of a target. Currently, only frameworks are
/// supported. Note that framework dependencies are also used by `xcodegen` to
/// refer to static libraries (despite the fact that frameworks are dynamic).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    /// Represents a framework dependency.
    Framework {
        /// Name or path of framework to link.
        framework: String,

        /// Whether or not to include a copy of the framework within the app's
        /// bundle. This should be false for a static library because it's
        /// unnecessary to include the `lib*.a` file in the bundle and
        /// additionally, Apple might reject apps that do ship static libraries
        /// that are not linked to the executable.
        embed: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spec() {
        let project = Project {
            name: "Example".to_owned(),
            targets: vec![(
                "Example".to_owned(),
                Target {
                    product_type: ProductType::Application,
                    platform: vec![Platform::IOS].into_iter().collect(),
                    deployment_targets: vec![(Platform::IOS, "8.0".to_owned())]
                        .into_iter()
                        .collect(),
                    sources: vec![TargetSource {
                        path: "Sources".to_owned(),
                    }],
                    settings: vec![
                        (
                            "LIBRARY_SEARCH_PATHS".to_owned(),
                            "../x86_64-apple-ios/debug".to_owned(),
                        ),
                        ("OTHER_LDFLAGS".to_owned(), "-ObjC".to_owned()),
                    ]
                    .into_iter()
                    .collect(),
                    dependencies: vec![Dependency::Framework {
                        framework: "libexample.a".to_owned(),
                        embed: false,
                    }],
                },
            )]
            .into_iter()
            .collect(),
        };

        let result = serde_yaml::to_string(&project).unwrap();
        println!("Result: {}", result);
    }
}
