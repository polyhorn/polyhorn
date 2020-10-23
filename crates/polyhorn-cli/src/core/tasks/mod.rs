//! Tasks and context relevant for building and running on any platform.

mod install_dependencies;
mod install_target;

pub use install_dependencies::{Dependency, DependencyCheck, InstallDependencies};
pub use install_target::InstallTarget;
