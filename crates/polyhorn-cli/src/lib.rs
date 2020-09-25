pub mod commands;
pub mod core;
pub mod ios;
pub mod spec;

use clap::{AppSettings, Clap};

/// Polyhorn is a platform for rapidly building apps that run everywhere.
#[derive(Clap)]
#[clap(name = "polyhorn", version = "1.0", author = "Glacyr B.V.")]
#[clap(setting(AppSettings::ColoredHelp))]
pub struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    New(commands::New),
    Run(commands::Run),
    Watch(commands::Watch),
}

pub fn cli() {
    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::New(new) => new.main(),
        SubCommand::Run(run) => run.main(),
        SubCommand::Watch(watch) => watch.main(),
    }
}
