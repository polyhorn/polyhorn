use super::{IOSContext, IOSError};
use crate::core::{CargoBuild, Manager, Task};

/// This tasks builds the runtime library for the given target and with the
/// given profile.
pub struct BuildRuntimeLibrary {
    /// The target for which this library will be built.
    pub target: &'static str,

    /// The profile to pass to Cargo, e.g. `debug` or `release`.
    pub profile: &'static str,
}

impl Task for BuildRuntimeLibrary {
    type Context = IOSContext;
    type Error = IOSError;

    fn verb(&self) -> &str {
        "Building"
    }

    fn message(&self) -> &str {
        "runtime library"
    }

    fn detail(&self) -> &str {
        "for iOS"
    }

    fn run(&self, mut context: IOSContext, _manager: &mut Manager) -> Result<IOSContext, IOSError> {
        eprintln!("");

        let name = CargoBuild::new(&context.config.manifest_dir.join("Cargo.toml"))
            .crate_type("staticlib")
            .release(self.profile == "release")
            .target(self.target)
            .build()?;

        context.products.insert(
            self.target.to_owned(),
            context.config.manifest_dir.join(format!(
                "target/{}/{}/lib{}.a",
                self.target, self.profile, name
            )),
        );

        Ok(context)
    }
}
