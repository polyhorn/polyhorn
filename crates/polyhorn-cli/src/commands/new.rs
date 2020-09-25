use clap::Clap;
use std::fs::create_dir;
use std::io::{Error, Write};
use std::path::PathBuf;

/// Creates a new Polyhorn app in the given directory.
#[derive(Clap)]
pub struct New {
    path: PathBuf,
}

impl New {
    pub fn main(&self) {
        // TODO: add error.
        let _ = create_dir(&self.path);

        self.write_cargo().unwrap();
        self.write_assets().unwrap();
        self.write_polyhorn().unwrap();
        self.write_build().unwrap();

        let mut src = self.path.clone();
        src.push("src");
        let _ = create_dir(&src);

        self.write_lib().unwrap();
    }

    fn write_assets(&self) -> Result<(), Error> {
        let mut assets = self.path.clone();
        assets.push("assets");
        let _ = create_dir(&assets);

        assets.push(".gitkeep");
        std::fs::File::create(assets)?;

        Ok(())
    }

    fn write_cargo(&self) -> Result<(), Error> {
        let mut cargo = self.path.clone();
        cargo.push("Cargo.toml");

        let mut file = std::fs::File::create(cargo).unwrap();
        file.write_fmt(format_args!(
            include_str!("../../template/Cargo.toml.tmpl"),
            "example"
        ))
    }

    fn write_polyhorn(&self) -> Result<(), Error> {
        let mut cargo = self.path.clone();
        cargo.push("Polyhorn.toml");

        let mut file = std::fs::File::create(cargo).unwrap();
        file.write_fmt(format_args!(include_str!(
            "../../template/Polyhorn.toml.tmpl"
        ),))
    }

    fn write_build(&self) -> Result<(), Error> {
        let mut build = self.path.clone();
        build.push("build.rs");

        let mut file = std::fs::File::create(build).unwrap();
        file.write_fmt(format_args!(include_str!("../../template/build.rs.tmpl")))
    }

    fn write_lib(&self) -> Result<(), Error> {
        let mut lib = self.path.clone();
        lib.push("src");
        lib.push("lib.rs");

        let mut file = std::fs::File::create(lib).unwrap();
        file.write_fmt(format_args!(include_str!("../../template/src/lib.rs.tmpl")))
    }
}
