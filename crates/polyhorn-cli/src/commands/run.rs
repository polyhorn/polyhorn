use ansi_term::Colour::Red;
use clap::Clap;
use path_absolutize::Absolutize;
use std::path::Path;

use super::Platform;
use crate::spec::{Error, Spec};

/// Runs the app on a device or simulator.
#[derive(Clap)]
pub struct Run {
    #[clap(subcommand)]
    platform: Platform,
}

impl Run {
    /// Implementation of the `polyhorn run` command that delegates its work to
    /// one of the platform-specific implementations.
    pub fn main(&self, manifest_path: &Path) {
        let spec = match Spec::open(manifest_path) {
            Ok(spec) => spec,
            Err(Error::TOML(error)) => {
                eprintln!(
                    "{}: could not read manifest: {}",
                    Red.bold().paint("error"),
                    error
                );
                std::process::abort();
            }
            Err(Error::IO(_)) => {
                eprintln!(
                    "{}: could not find file: {:?}",
                    Red.bold().paint("error"),
                    manifest_path
                        .absolutize_virtually(std::env::current_dir().unwrap())
                        .unwrap(),
                );
                std::process::abort();
            }
        };

        let manifest_path = std::fs::canonicalize(manifest_path).unwrap();

        let mut manifest_dir = manifest_path.clone();
        manifest_dir.pop();

        let mut target_dir = manifest_dir.clone();
        target_dir.push("target");

        let config = crate::Config {
            manifest_path,
            manifest_dir,
            target_dir,
            spec,
        };

        match self.platform {
            Platform::Android => crate::android::commands::run(config),
            Platform::IOS => crate::ios::commands::run(config),
        }
    }
}
