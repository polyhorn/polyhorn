use super::{IOSContext, IOSError};
use crate::core::{Manager, Task};
use crate::ios::simctl::Simctl;

/// This task installs an application on the iOS Simulator with the given
/// identifier.
pub struct InstallOnIOSSimulator {
    /// Configuration with which the application was built, which is used to
    /// locate its path.
    pub configuration: String,

    /// Identifier of the iOS Simulator on which the application will be
    /// installed.
    pub identifier: String,

    /// Name of the iOS Simulator that is shown to the user.
    pub name: String,
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
        &self.name
    }

    fn run(
        &self,
        context: Self::Context,
        _manager: &mut Manager,
    ) -> Result<Self::Context, Self::Error> {
        let mut simctl = Simctl::new();
        let target_dir = context.config.target_dir.join("polyhorn-ios");

        if let Err(error) = simctl.install(
            &self.identifier,
            target_dir.join(
                format!(
                    "derived-data/Build/Products/{}-iphonesimulator/{}.app",
                    self.configuration, context.config.spec.app.name
                )
                .as_str(),
            ),
        ) {
            return Err(IOSError::Simctl(error));
        }

        Ok(context)
    }
}
