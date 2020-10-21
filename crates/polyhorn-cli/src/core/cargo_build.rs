use cargo_metadata::MetadataCommand;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::change_crate_type;

/// Builder for cargo build commands.
#[derive(Default)]
pub struct CargoBuild {
    manifest_path: PathBuf,
    crate_type: Option<String>,
    target: Option<String>,
    release: bool,
}

impl CargoBuild {
    /// Returns a new builder for a cargo build command that operates on the
    /// manifest residing at the given path.
    pub fn new(manifest_path: &Path) -> CargoBuild {
        CargoBuild {
            manifest_path: manifest_path.to_owned(),
            ..Default::default()
        }
    }

    /// Builds a crate of the given type, overriding the type that is written in
    /// the manifest.
    pub fn crate_type(mut self, crate_type: &str) -> CargoBuild {
        self.crate_type = Some(crate_type.to_owned());
        self
    }

    /// Changes the profile of this build.
    pub fn release(mut self, release: bool) -> CargoBuild {
        self.release = release;
        self
    }

    /// Changes the target of this build.
    pub fn target(mut self, target: &str) -> CargoBuild {
        self.target = Some(target.to_owned());
        self
    }

    /// Executes the build and returns the crate name of the first target that
    /// matches the requested crate type for this build.
    pub fn build(self) -> Result<String> {
        let mut guard = None;

        if let Some(crate_type) = self.crate_type.as_ref() {
            guard.replace(change_crate_type(&self.manifest_path, crate_type)?);
        }

        let metadata = MetadataCommand::new()
            .manifest_path(&self.manifest_path)
            .exec()
            .unwrap();
        let root_id = metadata.resolve.unwrap().root.unwrap();
        let package = metadata
            .packages
            .iter()
            .find(|package| package.id == root_id)
            .unwrap();

        let mut command = Command::new("cargo");
        command.arg("build");
        command.arg("--manifest-path");
        command.arg(&self.manifest_path);

        if let Some(target) = self.target.as_ref() {
            command.arg("--target");
            command.arg(target);
        }

        if self.release {
            command.arg("--release");
        }

        match command.status()? {
            status if status.success() => {}
            status => {
                std::process::exit(status.code().unwrap());
            }
        }

        let mut targets = package.targets.iter();

        let target = if let Some(crate_type) = self.crate_type.as_ref() {
            targets.find(|target| target.kind == vec![crate_type.to_owned()])
        } else {
            targets.next()
        };

        Ok(target.unwrap().name.replace("-", "_"))
    }
}
