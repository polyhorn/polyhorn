use super::{IOSContext, IOSError};
use crate::core::{Manager, Task};
use crate::ios::simctl::Simctl;

/// This task launches the application on an iOS Simulator with the given
/// identifier.
pub struct RunOnIOSSimulator {
    /// The identifier of the iOS Simulator on which to launch the application.
    pub identifier: String,

    /// Name of the iOS Simulator.
    pub name: String,
}

impl Task for RunOnIOSSimulator {
    type Context = IOSContext;
    type Error = IOSError;

    fn verb(&self) -> &str {
        "Running"
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

        eprintln!("");

        if let Err(error) = simctl.launch(
            &self.identifier,
            &context.config.spec.app.ios.bundle_identifier,
        ) {
            return Err(IOSError::Simctl(error));
        }

        Ok(context)
    }
}
