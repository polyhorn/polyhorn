//! CLI that makes it easy to work with Polyhorn projects. Specifically, the CLI
//! takes care of building, running and distributing cross-platform apps written
//! with Polyhorn.

#![warn(missing_docs)]

pub mod android;
pub mod commands;
pub mod core;
pub mod ios;
pub mod spec;
pub mod template;
pub mod test;

use clap::{AppSettings, Clap};
use std::path::PathBuf;

/// Contains a configuration that is passed to the implementation of each
/// (relevant) CLI command. A notable exception is `polyhorn new`, which doesn't
/// require a pre-existing `Polyhorn.toml` manifest file.
#[derive(Debug)]
pub struct Config {
    /// Contains the path of the directory that stores the `Polyhorn.toml` file.
    pub manifest_dir: PathBuf,

    /// Contains the path of the `Polyhorn.toml` file itself.
    pub manifest_path: PathBuf,

    /// Contains the path of the directory that build products should be written
    /// to.
    pub target_dir: PathBuf,

    /// Contains the specification that is read from the `Polyhorn.toml` file.
    pub spec: spec::Spec,
}

/// Polyhorn is a platform for rapidly building apps that run everywhere.
#[derive(Clap)]
#[clap(name = "polyhorn", version = env!("CARGO_PKG_VERSION"), author = "Glacyr B.V.")]
#[clap(setting(AppSettings::ColoredHelp))]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,

    #[clap(long = "manifest-path", default_value = "Polyhorn.toml")]
    manifest_path: PathBuf,
}

/// Subcommands provided by the Polyhorn CLI.
#[derive(Clap)]
enum SubCommand {
    /// Creates a new Polyhorn app in the given directory.
    Init(commands::Init),

    /// Runs the app on a device or simulator.
    Run(commands::Run),

    /// Tests the app on a device or simulator.
    Test(commands::Test),
}

/// Entry point of the CLI that is used by the main `polyhorn` package. The
/// `polyhorn-cli` package itself doesn't provide a binary. This is because we
/// want to ship a library named `polyhorn` and a CLI with the same name (but
/// different crates obviously can't share the same name). So instead, the
/// library gets to be `polyhorn` and comes with a binary target that simply
/// calls `polyhorn_cli::cli()`.
pub fn cli() {
    #[cfg(windows)]
    let enabled = ansi_term::enable_ansi_support();

    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Init(init) => init.main(),
        SubCommand::Run(run) => run.main(&opts.manifest_path),
        SubCommand::Test(test) => test.main(&opts.manifest_path),
    }
}
