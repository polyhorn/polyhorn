mod new;
mod run;
mod watch;

pub use new::New;
pub use run::Run;
pub use watch::Watch;

use clap::Clap;

#[derive(Clap)]
pub enum Platform {
    IOS,
    Android,
}
