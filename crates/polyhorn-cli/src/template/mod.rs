//! Utilities for quickly building a source tree from a series of templates.

use serde::Serialize;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use tinytemplate::TinyTemplate;

use crate::spec::Spec;

/// Represents the contents of a source file that needs to be generated.
pub enum SourceFileContents {
    /// Represents a template. The template will be invoked with the spec
    /// contained in `Polyhorn.toml`.
    Template(&'static str),

    /// Copies the contents of a file from the given slice without any string
    /// interpolation.
    Copy(&'static [u8]),
}

/// A single source file that is generated.
pub struct SourceFile {
    name: String,
    contents: SourceFileContents,
}

/// A source tree that is used to queue files that need to be generated.
pub struct SourceTree {
    files: Vec<SourceFile>,
}

/// Context that is passed to the template engine that is used for generating
/// source files dynamically.
#[derive(Serialize)]
pub struct Context<'a> {
    spec: &'a Spec,
}

/// An iterator over the files that a source tree generates one-by-one.
pub struct SourceTreeIter<'a> {
    files: &'a [SourceFile],
    current: usize,
    context: Context<'a>,
    destination_path: &'a Path,
}

/// Represents an error that occurs during source tree generation.
#[derive(Debug)]
pub enum Error {
    /// Contains an error returned by the template engine invocation.
    Template(tinytemplate::error::Error),

    /// Contains an IO error that is encountered when writing the generated
    /// source files to disk.
    IO(std::io::Error),
}

impl<'a> Iterator for SourceTreeIter<'a> {
    type Item = Result<(), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.files.len() {
            return None;
        }

        let file = &self.files[self.current];
        self.current += 1;

        let mut buf = self.destination_path.to_path_buf();
        buf.push(file.name.to_owned());

        {
            let mut buf = buf.clone();
            buf.pop();

            let _ = create_dir_all(buf);
        }

        match file.contents {
            SourceFileContents::Copy(contents) => {
                let mut file = match File::create(buf) {
                    Ok(file) => file,
                    Err(error) => return Some(Err(Error::IO(error))),
                };

                if let Err(error) = file.write_all(contents) {
                    return Some(Err(Error::IO(error)));
                }
            }
            SourceFileContents::Template(template) => {
                let mut engine = TinyTemplate::new();

                if let Err(error) = engine.add_template("template", template) {
                    return Some(Err(Error::Template(error)));
                }

                let contents = match engine.render("template", &self.context) {
                    Ok(contents) => contents,
                    Err(error) => return Some(Err(Error::Template(error))),
                };

                let mut file = match File::create(buf) {
                    Ok(file) => file,
                    Err(error) => return Some(Err(Error::IO(error))),
                };

                if let Err(error) = file.write_all(contents.as_bytes()) {
                    return Some(Err(Error::IO(error)));
                }
            }
        }

        Some(Ok(()))
    }
}

impl SourceTree {
    /// Creates a new empty source tree that can be written to disk.
    pub fn new() -> SourceTree {
        SourceTree { files: vec![] }
    }

    /// Queues a copy of the given contents to a file with the given name.
    pub fn copy(&mut self, name: &str, contents: &'static [u8]) {
        self.files.push(SourceFile {
            name: name.to_owned(),
            contents: SourceFileContents::Copy(contents),
        })
    }

    /// Queues an invocation of the given template that writes the result to a
    /// file with the given name.
    pub fn template(&mut self, name: &str, contents: &'static str) {
        self.files.push(SourceFile {
            name: name.to_owned(),
            contents: SourceFileContents::Template(contents),
        });
    }

    /// Returns the number of files that are queued for generation.
    pub fn len(&self) -> usize {
        self.files.len()
    }

    /// Returns an iterator that generates the source files of this source tree
    /// and writes them to the given destination path one-by-one.
    pub fn generate<'a>(
        &'a self,
        spec: &'a Spec,
        destination_path: &'a Path,
    ) -> SourceTreeIter<'a> {
        SourceTreeIter {
            files: &self.files,
            current: 0,
            context: Context { spec },
            destination_path,
        }
    }
}
