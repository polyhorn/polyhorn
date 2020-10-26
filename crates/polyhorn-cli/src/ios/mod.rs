//! Tasks and types for iOS-specific operations.

pub mod commands;
pub mod infoplist;
pub mod tasks;
pub mod xcassets;
pub mod xcodegen;

mod spec;
pub use spec::Spec;
