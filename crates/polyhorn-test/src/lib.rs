#![warn(missing_docs)]

//! Polyhorn tests are the most ergonomic way to test Polyhorn apps.

mod app;
mod automator;
mod channel;
mod client;
pub mod inventory;

pub use app::App;
pub use automator::Automator;
