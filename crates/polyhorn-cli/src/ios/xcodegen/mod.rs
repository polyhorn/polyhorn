use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Project {
    /// Name of the generated project.
    pub name: String,

    /// The list of targets in the project mapped by name.
    pub targets: HashMap<String, Target>,
}

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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ProductType {
    #[serde(rename = "application")]
    Application,

    #[serde(rename = "application-on-demand-install-capable")]
    ApplicationOnDemandInstallCapable,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Platform {
    #[serde(rename = "iOS")]
    IOS,

    #[serde(rename = "macOS")]
    MacOS,

    #[serde(rename = "tvOS")]
    TVOS,

    #[serde(rename = "watchOS")]
    WatchOS,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TargetSource {
    /// The path of the source file or directory.
    pub path: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    Framework { framework: String, embed: bool },
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
