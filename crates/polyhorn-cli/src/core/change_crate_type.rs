use std::fs::{remove_file, File, OpenOptions};
use std::io::{copy, Read, Result, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use toml::Value;

pub struct ChangeCrateTypeGuard {
    source: String,
    backup_path: PathBuf,
    source_file: File,
}

impl ChangeCrateTypeGuard {
    pub fn new(source_path: &Path, crate_type: &str) -> Result<ChangeCrateTypeGuard> {
        // We start by making a backup of the file at the given path.
        let mut backup_path = source_path.to_path_buf();
        backup_path.set_extension("toml.backup");

        // Create the backup file, but only if it doesn't already exist.
        let mut backup_file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&backup_path)
            .unwrap();

        // Open the source file, but don't create if it doesn't already exist.
        let mut source_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&source_path)
            .unwrap();

        // Copy the contents of the source file to the backup file.
        copy(&mut source_file, &mut backup_file).unwrap();
        backup_file.flush().unwrap();

        // Now, we overwrite the source using toml_edit.
        let mut source = String::new();
        source_file.seek(SeekFrom::Start(0)).unwrap();
        source_file.read_to_string(&mut source).unwrap();

        let mut document = source.parse::<Value>().unwrap();
        document
            .as_table_mut()
            .unwrap()
            .entry("lib")
            .or_insert(Value::Table(Default::default()))
            .as_table_mut()
            .unwrap()
            .insert("crate-type".to_owned(), vec![crate_type.to_owned()].into());

        source_file.seek(SeekFrom::Start(0)).unwrap();
        source_file.set_len(0).unwrap();
        source_file
            .write_all(document.to_string().as_bytes())
            .unwrap();
        source_file.flush()?;

        Ok(ChangeCrateTypeGuard {
            source,
            backup_path,
            source_file,
        })
    }
}

impl Drop for ChangeCrateTypeGuard {
    fn drop(&mut self) {
        // Here, we undo any changes made to the manifest.
        self.source_file.seek(SeekFrom::Start(0)).unwrap();
        self.source_file.set_len(0).unwrap();
        self.source_file.write_all(self.source.as_bytes()).unwrap();
        self.source_file.flush().unwrap();

        remove_file(&self.backup_path).unwrap();
    }
}

/// Changes the crate type of the manifest at the given path and returns a guard
/// that restores any changes made to the manifest when it leaves scope.
pub fn change_crate_type(path: &Path, crate_type: &str) -> Result<ChangeCrateTypeGuard> {
    ChangeCrateTypeGuard::new(path, crate_type)
}
