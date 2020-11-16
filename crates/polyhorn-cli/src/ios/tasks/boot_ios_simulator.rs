use simctl::Device;

use super::{IOSContext, IOSError};
use crate::core::{Manager, Task};

/// This tasks boots an iOS Simulator with the given identifier.
pub struct BootIOSSimulator {
    /// This is the iOS Simulator that will be booted.
    pub device: Device,
}

impl Task for BootIOSSimulator {
    type Context = IOSContext;
    type Error = IOSError;

    fn verb(&self) -> &str {
        "Booting"
    }

    fn message(&self) -> &str {
        "iOS Simulator"
    }

    fn detail(&self) -> &str {
        &self.device.name
    }

    fn run(
        &self,
        context: Self::Context,
        _manager: &mut Manager,
    ) -> Result<Self::Context, Self::Error> {
        let _ = self.device.boot();

        Ok(context)
    }
}
