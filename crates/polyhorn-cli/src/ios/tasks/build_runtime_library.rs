use ansi_term::Colour::Red;
use cargo::core::compiler::{CompileKind, CompileMode, CompileTarget, CrateType};
use cargo::core::manifest::TargetKind;
use cargo::core::Workspace;
use cargo::ops::{compile, CompileOptions};
use cargo::util::interning::InternedString;
use cargo::util::Config;
use std::path::{Path, PathBuf};

use super::{IOSContext, IOSError};
use crate::core::{Manager, Task};

/// This tasks builds the runtime library for the given target and with the
/// given profile.
pub struct BuildRuntimeLibrary {
    /// The target for which this library will be built.
    pub target: &'static str,

    /// The profile to pass to Cargo, e.g. `debug` or `release`.
    pub profile: &'static str,
}

impl BuildRuntimeLibrary {
    /// Utility function that wraps the commands that are sent to Cargo.
    pub fn build(&self, manifest_path: &Path) -> Result<PathBuf, IOSError> {
        let mut config = Config::default().unwrap();
        config
            .configure(0, false, None, false, false, false, &None, &[], &[])
            .unwrap();

        let mut workspace = Workspace::new(manifest_path, &config).unwrap();

        for target in workspace
            .current_mut()
            .unwrap()
            .manifest_mut()
            .targets_mut()
        {
            match target.kind() {
                TargetKind::Lib(_) => {
                    target.set_kind(TargetKind::Lib(vec![CrateType::Staticlib]));
                }
                _ => {}
            }
        }

        let name = workspace
            .current()
            .unwrap()
            .targets()
            .iter()
            .find(|target| matches!(target.kind(), TargetKind::Lib(_)))
            .unwrap()
            .crate_name();

        let kind = CompileKind::Target(CompileTarget::new(self.target).unwrap());

        let mut options = CompileOptions::new(&config, CompileMode::Build).unwrap();
        options.build_config.requested_profile = InternedString::new(self.profile);
        options.build_config.requested_kinds = vec![kind.clone()];

        match compile(&workspace, &options) {
            Ok(compilation) => Ok(compilation
                .root_output
                .get(&kind)
                .unwrap()
                .join(format!("lib{}.a", name))),
            Err(error) => {
                eprintln!("{}: {:?}", Red.bold().paint("error"), error);
                Err(IOSError::CompilationFailure)
            }
        }
    }
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
        // Then we locate the Cargo manifest.
        let mut manifest_path = context.config.manifest_dir.clone();
        manifest_path.push("Cargo.toml");

        // Cargo wants to start at a new line.
        eprintln!("");

        let result = self.build(&manifest_path);

        match result {
            Ok(path) => {
                context.products.insert(self.target.to_owned(), path);
                Ok(context)
            }
            Err(error) => Err(error),
        }
    }
}
