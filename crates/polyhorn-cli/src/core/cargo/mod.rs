//! Wrapper around Cargo that is currently still used by the iOS implementation.

use cargo_metadata::{Message, Metadata, MetadataCommand};
use std::io::{BufReader, Error};
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Reads the metadata stored in the `Cargo.toml` file of the current working
/// directory. Note that this does not support `--manifest-path`.
pub fn metadata() -> Result<Metadata, Error> {
    let command = MetadataCommand::new();
    Ok(command.exec().unwrap())
}

/// Build function that is currently still used by the iOS implementation. Note
/// that this does not support `--manifest-path`.
pub fn build() -> Result<PathBuf, Error> {
    let metadata = metadata()?;
    let root_id = metadata.resolve.unwrap().root.unwrap();

    let mut command = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("x86_64-apple-ios")
        .arg("--message-format=json")
        .stdout(Stdio::piped())
        .spawn()?;

    let reader = BufReader::new(command.stdout.take().unwrap());

    let mut path = None;

    for message in Message::parse_stream(reader) {
        match message.unwrap() {
            Message::CompilerArtifact(mut artifact) if artifact.package_id == root_id => {
                path = artifact.filenames.pop();
            }
            _ => {}
        }
    }

    let output = command.wait().unwrap();

    assert!(output.success());

    Ok(path.unwrap())
}
