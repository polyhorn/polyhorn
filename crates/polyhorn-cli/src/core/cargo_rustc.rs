use cargo_metadata::{Message, MetadataCommand};
use std::io::{BufReader, Result};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Builder for cargo build commands.
#[derive(Default)]
pub struct CargoRustc {
    manifest_path: PathBuf,
    cfg: Option<String>,
    crate_type: Option<String>,
    target: Option<String>,
    profile: Option<String>,
    flags: Vec<String>,
    release: bool,
}

impl CargoRustc {
    /// Returns a new builder for a cargo build command that operates on the
    /// manifest residing at the given path.
    pub fn new(manifest_path: &Path) -> CargoRustc {
        CargoRustc {
            manifest_path: manifest_path.to_owned(),
            ..Default::default()
        }
    }

    /// Builds a crate with the given cfg.
    pub fn cfg(mut self, cfg: &str) -> CargoRustc {
        self.cfg = Some(cfg.to_owned());
        self
    }

    /// Builds a crate with the given profile.
    pub fn profile(mut self, profile: &str) -> CargoRustc {
        self.profile = Some(profile.to_owned());
        self
    }

    /// Builds a crate of the given type, overriding the type that is written in
    /// the manifest.
    pub fn crate_type(mut self, crate_type: &str) -> CargoRustc {
        self.crate_type = Some(crate_type.to_owned());
        self
    }

    /// Changes the profile of this build.
    pub fn release(mut self, release: bool) -> CargoRustc {
        self.release = release;
        self
    }

    /// Changes the target of this build.
    pub fn target(mut self, target: &str) -> CargoRustc {
        self.target = Some(target.to_owned());
        self
    }

    /// Changes the flags that are passed to `rustc`.
    pub fn flags(mut self, flags: &[&str]) -> CargoRustc {
        self.flags = flags.into_iter().map(|flag| flag.to_string()).collect();
        self
    }

    /// Executes the build and returns the crate name of the first target that
    /// matches the requested crate type for this build.
    pub fn build(self) -> Result<PathBuf> {
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
        command.arg("rustc");
        command.arg("--manifest-path");
        command.arg(&self.manifest_path);

        if let Some(target) = self.target.as_ref() {
            command.arg("--target");
            command.arg(target);
        }

        if let Some(profile) = self.profile.as_ref() {
            command.arg("--profile");
            command.arg(profile);
        }

        if self.release {
            command.arg("--release");
        }

        command.arg("--message-format=json-render-diagnostics");

        command.arg("--");

        if let Some(cfg) = self.cfg.as_ref() {
            command.arg("--cfg");
            command.arg(cfg);
        }

        if let Some(crate_type) = self.crate_type.as_ref() {
            command.arg("--crate-type");
            command.arg(crate_type);
        }

        command.args(&self.flags);

        command.stdout(Stdio::piped());
        command.stderr(Stdio::inherit());

        let mut process = command.spawn().unwrap();

        let reader = BufReader::new(process.stdout.take().unwrap());

        let mut path = None;

        for message in Message::parse_stream(reader) {
            match message.unwrap() {
                Message::CompilerArtifact(artifact) => {
                    if artifact.package_id == package.id {
                        path = artifact.filenames.first().cloned();
                    }
                }
                _ => {}
            }
        }

        match process.wait()? {
            status if status.success() => {}
            status => {
                std::process::exit(status.code().unwrap());
            }
        }

        Ok(path.unwrap())
    }
}
