use std::process::{Command, Stdio};

use crate::core::{Manager, Task};

/// This task installs a target with the given name using rustup, if necessary.
pub struct InstallTarget(pub &'static str);

impl Task for InstallTarget {
    type Context = ();
    type Error = std::io::Error;

    fn verb(&self) -> &str {
        "Installing"
    }

    fn message(&self) -> &str {
        "target"
    }

    fn detail(&self) -> &str {
        self.0
    }

    fn run(
        &self,
        context: Self::Context,
        _manager: &mut Manager,
    ) -> Result<Self::Context, Self::Error> {
        Command::new("rustup")
            .arg("target")
            .arg("add")
            .arg(self.0)
            .stderr(Stdio::null())
            .spawn()?
            .wait()?;

        Ok(context)
    }
}
