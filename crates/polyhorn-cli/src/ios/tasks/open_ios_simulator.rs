use std::process::{Command, Stdio};

use super::{IOSContext, IOSError};
use crate::core::{Manager, Task};

/// This task opens the iOS Simulator GUI, which is not open by default when
/// booting a (new) simulator.
pub struct OpenIOSSimulator;

impl Task for OpenIOSSimulator {
    type Context = IOSContext;
    type Error = IOSError;

    fn verb(&self) -> &str {
        "Opening"
    }

    fn message(&self) -> &str {
        "iOS Simulator"
    }

    fn detail(&self) -> &str {
        ""
    }

    fn run(
        &self,
        context: Self::Context,
        _manager: &mut Manager,
    ) -> Result<Self::Context, Self::Error> {
        Command::new("open")
            .arg("-a")
            .arg("Simulator.app")
            .stdout(Stdio::null())
            .current_dir(context.config.target_dir.join("polyhorn-ios"))
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        Ok(context)
    }
}
