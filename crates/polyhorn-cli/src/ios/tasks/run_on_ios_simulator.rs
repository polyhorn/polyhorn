use simctl::Device;

use super::{IOSContext, IOSError};
use crate::core::{Manager, Task};

/// This task launches the application on an iOS Simulator with the given
/// identifier.
pub struct RunOnIOSSimulator {
    /// The iOS Simulator on which to launch the application.
    pub device: Device,
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
        &self.device.name
    }

    fn run(
        &self,
        context: Self::Context,
        _manager: &mut Manager,
    ) -> Result<Self::Context, Self::Error> {
        eprintln!("");

        self.device
            .launch(&context.config.spec.app.ios.bundle_identifier)
            .exec()?;

        Ok(context)
    }
}
