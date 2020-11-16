use super::{IOSContext, IOSError};
use crate::core::{CargoRustc, Manager, Task};

/// This tasks builds the runtime library for the given target and with the
/// given profile.
pub struct BuildRuntimeLibraryV2 {
    /// Custom cfg to use when building this library.
    pub cfg: &'static str,

    /// The target for which this library will be built.
    pub target: &'static str,

    /// The profile to pass to Cargo, e.g. `debug` or `release`.
    pub profile: &'static str,

    /// Additional flags passed to `rustc`.
    pub flags: &'static [&'static str],
}

impl Task for BuildRuntimeLibraryV2 {
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

        let path = CargoRustc::new(&context.config.manifest_dir.join("Cargo.toml"))
            .crate_type("staticlib")
            .cfg(self.cfg)
            .profile(self.profile)
            .release(self.profile == "release")
            .target(self.target)
            .flags(self.flags)
            .build()?;

        context.products.insert(self.target.to_owned(), path);

        Ok(context)
    }
}
