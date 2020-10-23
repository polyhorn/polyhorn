use ansi_term::Color;
use dialoguer::Confirm;
use std::io::Error;
use std::process::{Command, Stdio};

use crate::core::{Manager, Task};

/// Check that will be used to determine if a dependency is installed.
pub struct DependencyCheck {
    /// Command that will be run to check if a dependency is installed.
    pub command: Vec<String>,
}

/// Dependency that must be installed.
pub struct Dependency {
    /// Name of this dependency that will be shown to the user.
    pub name: String,

    /// Check that will be used to determine if this dependency is installed.
    pub check: DependencyCheck,

    /// Command that will be executed to install this dependency if necessary.
    pub install_command: Vec<String>,
}

impl Dependency {
    /// Returns a new dependency on a CLI with the given commands to check its
    /// existence and install it if necessary, respectively.
    pub fn cli(name: &str, check_command: &[&str], install_command: &[&str]) -> Dependency {
        Dependency {
            name: name.to_owned(),
            check: DependencyCheck {
                command: check_command.iter().map(|&arg| arg.to_owned()).collect(),
            },
            install_command: install_command.iter().map(|&arg| arg.to_owned()).collect(),
        }
    }
}

/// This task checks if all dependencies exist and if necessary, installs the
/// dependencies that weren't found.
pub struct InstallDependencies {
    /// Dependencies that need to be installed.
    pub dependencies: Vec<Dependency>,
}

impl InstallDependencies {
    fn check(&self) -> Vec<&Dependency> {
        self.dependencies
            .iter()
            .filter_map(|dependency| {
                let mut command = Command::new(&dependency.check.command[0]);
                command.args(&dependency.check.command[1..]);
                let output = command.output().ok();

                match output.map(|output| output.status.success()) {
                    Some(true) => None,
                    _ => Some(dependency),
                }
            })
            .collect()
    }
}

impl Task for InstallDependencies {
    type Context = ();
    type Error = Error;

    fn verb(&self) -> &str {
        "Checking"
    }

    fn message(&self) -> &str {
        "dependencies"
    }

    fn detail(&self) -> &str {
        ""
    }

    fn run(
        &self,
        context: Self::Context,
        _manager: &mut Manager,
    ) -> Result<Self::Context, Self::Error> {
        let missing = self.check();

        if missing.is_empty() {
            return Ok(context);
        }

        eprintln!("\n\nThe following dependencies haven't been found and need to be installed:\n",);

        for dependency in missing.iter() {
            eprintln!(
                "    {} {}",
                Color::Cyan.bold().paint(&dependency.name),
                Color::Fixed(8).paint(format!("({})", dependency.install_command.join(" ")))
            );
        }

        eprintln!("");

        if Confirm::new()
            .with_prompt("Do you want Polyhorn to install these dependencies automatically?")
            .interact()?
        {
            for dependency in missing {
                eprintln!(
                    "\nInstalling {} ...\n",
                    Color::Cyan.bold().paint(&dependency.name),
                );
                let mut command = Command::new(&dependency.install_command[0]);
                command.args(&dependency.install_command[1..]);
                command.stdout(Stdio::inherit());
                command.stderr(Stdio::inherit());

                match command.status() {
                    Ok(status) if status.success() => {}
                    Ok(status) => match status.code() {
                        Some(code) => std::process::exit(code),
                        None => std::process::exit(1),
                    },
                    Err(_) => {}
                }

                eprint!("");
            }
        } else {
            std::process::exit(1);
        }

        eprintln!("");

        Ok(context)
    }
}
