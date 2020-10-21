use super::{IOSContext, IOSError};
use crate::core::{Manager, Task};
use crate::ios::simctl::Simctl;

/// This tasks boots an iOS Simulator with the given identifier.
pub struct BootIOSSimulator {
    /// This is the name of the iOS Simulator that will be launched.
    pub name: String,

    /// This is the identifier of the iOS Simulator that will be launched.
    pub identifier: String,
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
        &self.name
    }

    fn run(
        &self,
        context: Self::Context,
        _manager: &mut Manager,
    ) -> Result<Self::Context, Self::Error> {
        let mut simctl = Simctl::new();

        if let Err(error) = simctl.boot(&self.identifier) {
            return Err(IOSError::Simctl(error));
        }

        Ok(context)
    }
}
