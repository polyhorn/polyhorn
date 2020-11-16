//! Types and implementations to write snapshots to disk.

use fs3::FileExt;
use serde::Serialize;
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Represents the metadata of a snapshot.
#[derive(Clone, Debug, Default, Serialize)]
pub struct Metadata {
    /// This is the device type. For example: "iPhone Xs".
    pub device: Option<String>,

    /// This is the operating system. For example: "iOS" or "Android".
    pub os: Option<String>,

    /// This is the version of the operating system. For example: "12.0". This
    /// version does not have to be semver compatible.
    pub os_version: Option<String>,

    /// This is the appearance of the operating system. For example: "dark" or
    /// "light".
    pub os_appearance: Option<String>,

    /// This is the name of the test itself.
    pub test_name: Option<String>,

    /// This is the name of the snapshot.
    pub snapshot_name: Option<String>,
}

trait Sha1Ext {
    fn tag(&mut self, prefix: &str, value: Option<&String>);
}

impl Sha1Ext for Sha1 {
    fn tag(&mut self, prefix: &str, value: Option<&String>) {
        if let Some(value) = value {
            self.update(prefix.as_bytes());
            self.update(format!("{}", value.len()).as_bytes());
            self.update(value.as_bytes());
        }
    }
}

impl Metadata {
    /// Computes a sha-1 digest for this metadata.
    pub fn digest(&self) -> Digest {
        let mut sha1 = Sha1::new();
        sha1.tag("device", self.device.as_ref());
        sha1.tag("os", self.os.as_ref());
        sha1.tag("os_version", self.os_version.as_ref());
        sha1.tag("os_appearance", self.os_appearance.as_ref());
        sha1.tag("test_name", self.test_name.as_ref());
        sha1.tag("snapshot_name", self.snapshot_name.as_ref());
        sha1.digest()
    }
}

/// Represents an output directory on disk.
pub struct Output {
    path: PathBuf,
    index_file: File,
    all_metadata: HashMap<String, Metadata>,
}

impl Output {
    /// Creates a new output directory with the given path.
    pub fn new<P>(path: P) -> Output
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_path_buf();
        let _ = std::fs::create_dir_all(&path);
        let index_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(path.join("Snapshots.toml"))
            .unwrap();

        index_file.lock_exclusive().unwrap();

        Output {
            path,
            index_file,
            all_metadata: HashMap::new(),
        }
    }

    /// Stores a snapshot with the given metadata in the output directory.
    pub fn store(&mut self, metadata: Metadata, screenshot: Vec<u8>) {
        let digest = metadata.digest().to_string();
        self.all_metadata.insert(digest.clone(), metadata);

        File::create(self.path.join(digest + ".png"))
            .unwrap()
            .write_all(&screenshot)
            .unwrap();
    }
}

impl Drop for Output {
    fn drop(&mut self) {
        self.index_file.set_len(0).unwrap();
        self.index_file
            .write_all(
                toml::to_string_pretty(&self.all_metadata)
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();

        self.index_file.unlock().unwrap();
    }
}
