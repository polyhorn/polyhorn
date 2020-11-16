//! Tasks and context specific for building and running on iOS.

use std::collections::HashMap;
use std::path::PathBuf;

mod boot_ios_simulator;
mod build_runtime_library;
mod build_runtime_library_v2;
mod build_xcodeproj;
mod create_universal_binary;
mod generate_xcassets;
mod generate_xcodeproj;
mod install_on_ios_simulator;
mod open_ios_simulator;
mod run_on_ios_simulator;

pub use boot_ios_simulator::BootIOSSimulator;
pub use build_runtime_library::BuildRuntimeLibrary;
pub use build_runtime_library_v2::BuildRuntimeLibraryV2;
pub use build_xcodeproj::BuildXcodeproj;
pub use create_universal_binary::CreateUniversalBinary;
pub use generate_xcassets::GenerateXcassets;
pub use generate_xcodeproj::GenerateXcodeproj;
pub use install_on_ios_simulator::InstallOnIOSSimulator;
pub use open_ios_simulator::OpenIOSSimulator;
pub use run_on_ios_simulator::RunOnIOSSimulator;

pub use crate::core::tasks::InstallTarget;
pub use crate::core::tasks::{Dependency, DependencyCheck, InstallDependencies};

use crate::core::{Manager, Task};
use crate::Config;

/// Context that gets passed through each task.
#[derive(Debug)]
pub struct IOSContext {
    /// The platform-independent configuration that gets passed to each command.
    pub config: Config,

    /// A map between iOS architectures and the path to the product for each.
    pub products: HashMap<String, PathBuf>,

    /// This will contain a path to the universal binary which is basically a
    /// bundle of architecture-specific static libraries.
    pub universal_binary_path: Option<PathBuf>,
}

/// Represents one of the iOS-specific tasks.
pub enum IOSTask {
    /// This task boots an iOS Simulator (if necessary).
    BootIOSSimulator(BootIOSSimulator),

    /// This task builds the runtime library for the given target and with the
    /// given profile.
    BuildRuntimeLibrary(BuildRuntimeLibrary),

    /// This task builds the runtime library for the given target and with the
    /// given profile.
    BuildRuntimeLibraryV2(BuildRuntimeLibraryV2),

    /// This task builds an .xcodeproj with a given scheme, configuration and
    /// destination.
    BuildXcodeproj(BuildXcodeproj),

    /// This task creates a universal binary from one or multiple
    /// architecture-specific static libraries for iOS.
    CreateUniversalBinary(CreateUniversalBinary),

    /// This task generates an Xcode-compatible assets catalog for the assets of
    /// the Polyhorn project that is being compiled and its dependencies.
    GenerateXcassets(GenerateXcassets),

    /// This task generates an xcodeproj.
    GenerateXcodeproj(GenerateXcodeproj),

    /// This task checks if a given set of dependencies exist and if necessary,
    /// installs the dependencies that weren't found.
    InstallDependencies(InstallDependencies),

    /// This task installs an application on the iOS Simulator with a given
    /// identifier.
    InstallOnIOSSimulator(InstallOnIOSSimulator),

    /// This task installs a target with a given name using rustup, if
    /// necessary.
    InstallTarget(InstallTarget),

    /// This task opens the iOS Simulator GUI, which is not open by default when
    /// booting a (new) simulator.
    OpenIOSSimulator(OpenIOSSimulator),

    /// This task launches the application on an iOS Simulator with a given
    /// identifier.
    RunOnIOSSimulator(RunOnIOSSimulator),
}

/// Represents an error that is returned by one of the iOS-specific tasks.
#[derive(Debug)]
pub enum IOSError {
    /// Returned by tasks that have not yet been implemented for a specific host
    /// operating system.
    UnsupportedHostOS(&'static str),

    /// Returned by the `BuildRuntimeLibrary` task when Cargo fails to build the
    /// runtime library (most likely because of an error in user code, e.g.
    /// syntax error).
    CompilationFailure,

    /// Returned by tasks when an io error occurs.
    IO(std::io::Error),

    /// Returned by tasks that interact with `simctl`, an Apple-provided utility
    /// to programmatically control the iOS Simulator, in the event that it
    /// returns an error.
    Simctl(simctl::Error),
}

impl From<std::io::Error> for IOSError {
    fn from(error: std::io::Error) -> Self {
        IOSError::IO(error)
    }
}

impl From<simctl::Error> for IOSError {
    fn from(error: simctl::Error) -> Self {
        IOSError::Simctl(error)
    }
}

impl Task for IOSTask {
    type Context = IOSContext;
    type Error = IOSError;

    fn verb(&self) -> &str {
        match self {
            IOSTask::BootIOSSimulator(task) => task.verb(),
            IOSTask::BuildRuntimeLibrary(task) => task.verb(),
            IOSTask::BuildRuntimeLibraryV2(task) => task.verb(),
            IOSTask::BuildXcodeproj(task) => task.verb(),
            IOSTask::CreateUniversalBinary(task) => task.verb(),
            IOSTask::GenerateXcassets(task) => task.verb(),
            IOSTask::GenerateXcodeproj(task) => task.verb(),
            IOSTask::InstallDependencies(task) => task.verb(),
            IOSTask::InstallOnIOSSimulator(task) => task.verb(),
            IOSTask::InstallTarget(task) => task.verb(),
            IOSTask::OpenIOSSimulator(task) => task.verb(),
            IOSTask::RunOnIOSSimulator(task) => task.verb(),
        }
    }

    fn message(&self) -> &str {
        match self {
            IOSTask::BootIOSSimulator(task) => task.message(),
            IOSTask::BuildRuntimeLibrary(task) => task.message(),
            IOSTask::BuildRuntimeLibraryV2(task) => task.message(),
            IOSTask::BuildXcodeproj(task) => task.message(),
            IOSTask::CreateUniversalBinary(task) => task.message(),
            IOSTask::GenerateXcassets(task) => task.message(),
            IOSTask::GenerateXcodeproj(task) => task.message(),
            IOSTask::InstallDependencies(task) => task.message(),
            IOSTask::InstallOnIOSSimulator(task) => task.message(),
            IOSTask::InstallTarget(task) => task.message(),
            IOSTask::OpenIOSSimulator(task) => task.message(),
            IOSTask::RunOnIOSSimulator(task) => task.message(),
        }
    }

    fn detail(&self) -> &str {
        match self {
            IOSTask::BootIOSSimulator(task) => task.detail(),
            IOSTask::BuildRuntimeLibrary(task) => task.detail(),
            IOSTask::BuildRuntimeLibraryV2(task) => task.detail(),
            IOSTask::BuildXcodeproj(task) => task.detail(),
            IOSTask::CreateUniversalBinary(task) => task.detail(),
            IOSTask::GenerateXcassets(task) => task.detail(),
            IOSTask::GenerateXcodeproj(task) => task.detail(),
            IOSTask::InstallDependencies(task) => task.detail(),
            IOSTask::InstallOnIOSSimulator(task) => task.detail(),
            IOSTask::InstallTarget(task) => task.detail(),
            IOSTask::OpenIOSSimulator(task) => task.detail(),
            IOSTask::RunOnIOSSimulator(task) => task.detail(),
        }
    }

    fn run(
        &self,
        context: Self::Context,
        manager: &mut Manager,
    ) -> Result<Self::Context, Self::Error> {
        match self {
            IOSTask::BootIOSSimulator(task) => task.run(context, manager),
            IOSTask::BuildRuntimeLibrary(task) => task.run(context, manager),
            IOSTask::BuildRuntimeLibraryV2(task) => task.run(context, manager),
            IOSTask::BuildXcodeproj(task) => task.run(context, manager),
            IOSTask::CreateUniversalBinary(task) => task.run(context, manager),
            IOSTask::GenerateXcassets(task) => task.run(context, manager),
            IOSTask::GenerateXcodeproj(task) => task.run(context, manager),
            IOSTask::InstallDependencies(task) => task
                .run((), manager)
                .map_err(|error| IOSError::IO(error))
                .map(|_| context),
            IOSTask::InstallOnIOSSimulator(task) => task.run(context, manager),
            IOSTask::InstallTarget(task) => task
                .run((), manager)
                .map_err(|error| IOSError::IO(error))
                .map(|_| context),
            IOSTask::OpenIOSSimulator(task) => task.run(context, manager),
            IOSTask::RunOnIOSSimulator(task) => task.run(context, manager),
        }
    }
}
