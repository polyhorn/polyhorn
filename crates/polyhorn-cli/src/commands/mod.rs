//! Implementations for platform-independent Polyhorn-cli commands.

mod new;
mod run;

pub use new::New;
pub use run::Run;

use clap::Clap;

/// Represents a choice between one of the supported platforms that Polyhorn
/// apps can be built for.
#[derive(Clap)]
pub enum Platform {
    /// Represents the iOS operating system that runs on iPhones, iPads etc.
    IOS,

    /// Represents the Android operating system that runs on most non-Apple
    /// smartphones.
    Android,
}
