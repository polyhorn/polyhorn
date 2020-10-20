use cargo_lock::Lockfile;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

fn manifest_path() -> Option<PathBuf> {
    let mut args = std::env::args();

    while let Some(arg) = args.next() {
        if arg == "--manifest-path" {
            return args.next().map(|path| Path::new(&path).to_path_buf());
        }
    }

    None
}

fn cli_version(path: &Path) -> Option<String> {
    let mut bytes = vec![];
    File::open(path.join(".polyhorn/.crates.toml"))
        .ok()?
        .read_to_end(&mut bytes)
        .ok()?;

    let crates = toml::from_slice::<Crates>(&bytes)?;
    let key = crates
        .v1
        .keys()
        .find(|key| key.starts_with("polyhorn-cli "));

    key?.split(" ")
        .skip(1)
        .next()
        .map(|version| version.to_owned())
}

fn install(path: &Path, version: Option<String>) {
    let mut command = Command::new("cargo");
    command.args(&[
        "install",
        "polyhorn-cli",
        "--features",
        "internal",
        "--root",
        ".polyhorn",
    ]);
    command.current_dir(path);

    if let Some(version) = version {
        command.args(&["--version", &format!("={}", version)]);
    }

    match command.status() {
        Ok(status) if status.success() => {}
        Ok(status) => std::process::exit(status.code().unwrap()),
        Err(error) => panic!("Couldn't install polyhorn-cli due to: {:?}", error),
    }
}

fn forward(path: &Path, args: &[String]) {
    let mut command = Command::new(path.join(".polyhorn/bin/polyhorn-cli"));
    command.args(args);
    command.current_dir(path);

    match command.status() {
        Ok(status) if status.success() => {}
        Ok(status) => std::process::exit(status.code().unwrap()),
        Err(error) => panic!("Couldn't invoke polyhorn-cli due to: {:?}", error),
    }
}

#[derive(Debug, Deserialize)]
struct Crates {
    v1: HashMap<String, Vec<String>>,
}

/// This is a small version manager for Polyhorn CLI.
fn main() {
    // We start by checking if the user has specified a manifest path.
    let manifest_dir = manifest_path()
        .map(|mut path| {
            path.pop();
            path
        })
        .unwrap_or(std::env::current_dir().unwrap())
        .canonicalize()
        .unwrap();

    // Then we check if a version of polyhorn (lib) is already present in the
    // Cargo.lock.
    let lockfile = Lockfile::load(manifest_dir.join("Cargo.lock"));
    let package = lockfile.ok().and_then(|file| {
        file.packages
            .into_iter()
            .find(|package| package.name.as_str() == "polyhorn")
    });

    match package {
        Some(package) => {
            // Check if we need to install a different version.
            match cli_version(&manifest_dir) {
                Some(version) if version == package.version.to_string() => {}
                _ => install(&manifest_dir, Some(package.version.to_string())),
            }

            let args = std::env::args().collect::<Vec<_>>();
            forward(&manifest_dir, &args.as_slice()[1..]);
        }
        None => {
            // We only install the CLI when the user runs `polyhorn new [name]`.
            let args = std::env::args().collect::<Vec<_>>();
            let args = args.iter().map(|arg| arg.as_str()).collect::<Vec<_>>();
            match args.as_slice() {
                &[_, "new", name] => {
                    // We start by creating a new directory.
                    let _ = std::fs::create_dir(name);

                    // Then, we download the CLI into this directory.
                    let manifest_dir = manifest_dir.join(name);
                    install(&manifest_dir, None);
                    forward(&manifest_dir, &["init".to_owned(), name.to_owned()]);
                }
                _ => {
                    eprintln!("Polyhorn is not installed in this directory. Create a new project with `polyhorn new [name]`.");
                    std::process::exit(1);
                }
            }
        }
    }
}
