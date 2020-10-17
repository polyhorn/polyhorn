//! Tasks and types for Android-specific operations.

pub mod commands;
pub mod tasks;

mod spec;
mod targets;

pub use spec::Spec;
pub use targets::Target;
