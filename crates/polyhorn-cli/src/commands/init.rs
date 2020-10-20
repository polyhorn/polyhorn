use clap::Clap;
use inflections::Inflect;
use std::fs::{create_dir, read_dir, File};
use std::io::{Error, Write};
use std::process::Command;

/// Creates a new Polyhorn app in the given directory.
#[derive(Clap)]
pub struct Init {
    name: String,
}

impl Init {
    /// Implementation of the platform-independent `polyhorn new` command.
    pub fn main(&self) {
        for entry in read_dir(std::env::current_dir().unwrap()).unwrap() {
            match entry.unwrap().file_name().to_str().unwrap() {
                ".polyhorn" => {}
                name => {
                    eprintln!(
                        "error: can't initialize new Polyhorn project due to existing file: {:?}",
                        name
                    );
                    std::process::abort();
                }
            }
        }

        self.write_assets().unwrap();
        self.write_build().unwrap();
        self.write_cargo().unwrap();
        self.write_gitignore().unwrap();
        self.write_polyhorn().unwrap();

        let _ = create_dir("src");

        self.write_lib().unwrap();

        assert!(Command::new("cargo")
            .arg("generate-lockfile")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
    }

    fn write_assets(&self) -> Result<(), Error> {
        let _ = create_dir("assets");
        File::create("assets/.gitkeep")?;

        Ok(())
    }

    fn write_cargo(&self) -> Result<(), Error> {
        let mut file = File::create("Cargo.toml").unwrap();
        file.write_fmt(format_args!(
            include_str!("../../template/Cargo.toml.tmpl"),
            name = self.name,
            version = env!("CARGO_PKG_VERSION"),
        ))
    }

    fn write_gitignore(&self) -> Result<(), Error> {
        let mut file = File::create(".gitignore").unwrap();
        file.write_fmt(format_args!(include_str!("../../template/.gitignore.tmpl"),))
    }

    fn write_polyhorn(&self) -> Result<(), Error> {
        let mut file = File::create("Polyhorn.toml").unwrap();
        file.write_fmt(format_args!(
            include_str!("../../template/Polyhorn.toml.tmpl"),
            title = self.name.to_title_case(),
            pascal = self.name.to_pascal_case(),
            snake = self.name.to_snake_case(),
        ))
    }

    fn write_build(&self) -> Result<(), Error> {
        let mut file = File::create("build.rs").unwrap();
        file.write_fmt(format_args!(include_str!("../../template/build.rs.tmpl")))
    }

    fn write_lib(&self) -> Result<(), Error> {
        let mut file = File::create("src/lib.rs").unwrap();
        file.write_fmt(format_args!(include_str!("../../template/src/lib.rs.tmpl")))
    }
}
