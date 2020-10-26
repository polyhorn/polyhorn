use simctl::Device;

use super::{IOSContext, IOSError};
use crate::core::{Manager, Task};

/// This task installs an application on the iOS Simulator with the given
/// identifier.
pub struct InstallOnIOSSimulator {
    /// Configuration with which the application was built, which is used to
    /// locate its path.
    pub configuration: String,

    /// iOS Simulator on which the application will be installed.
    pub device: Device,
}

impl Task for InstallOnIOSSimulator {
    type Context = IOSContext;
    type Error = IOSError;

    fn verb(&self) -> &str {
        "Installing"
    }

    fn message(&self) -> &str {
        "on iOS Simulator"
    }

    fn detail(&self) -> &str {
        &self.device.name
    }

    fn run(
        &self,
        context: Self::Context,
        _manager: &mut Manager,
    ) -> Result<Self::Context, Self::Error> {
        let target_dir = context.config.target_dir.join("polyhorn-ios");

        self.device.install(
            &target_dir.join(
                format!(
                    "derived-data/Build/Products/{}-iphonesimulator/{}.app",
                    self.configuration, context.config.spec.app.name
                )
                .as_str(),
            ),
        )?;

        Ok(context)
    }
}
