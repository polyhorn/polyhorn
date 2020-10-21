use std::process::{Command, Stdio};

use super::{IOSContext, IOSError};
use crate::core::{Manager, Task};

/// This task builds an .xcodeproj with the given scheme, configuration and
/// destination. It does not install the resulting product.
pub struct BuildXcodeproj {
    /// The scheme that will be used to build the xcodeproj.
    pub scheme: String,

    /// The configuration that will be used to build the xcodeproj.
    pub configuration: String,

    /// The destination platform that will be used to build the xcodeproj. E.g.
    /// "iOS Simulator".
    pub destination_platform: String,

    /// The destination name that will be used to build the xcodeproj. E.g.
    /// "iPhone SE (2nd generation)".
    pub destination_name: String,
}

impl Task for BuildXcodeproj {
    type Context = IOSContext;
    type Error = IOSError;

    fn verb(&self) -> &str {
        "Building"
    }

    fn message(&self) -> &str {
        "xcodeproj"
    }

    fn detail(&self) -> &str {
        ""
    }

    fn run(
        &self,
        context: Self::Context,
        _manager: &mut Manager,
    ) -> Result<Self::Context, Self::Error> {
        let target_dir = context.config.target_dir.join("polyhorn-ios");

        assert!(Command::new("xcrun")
            .arg("xcodebuild")
            .arg("-scheme")
            .arg(&self.scheme)
            .arg("-configuration")
            .arg(&self.configuration)
            .arg("-destination")
            .arg(format!(
                "platform={},name={}",
                self.destination_platform, self.destination_name
            ))
            .arg("-derivedDataPath")
            .arg("derived-data")
            .stdout(Stdio::null())
            .current_dir(target_dir)
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());

        Ok(context)
    }
}
