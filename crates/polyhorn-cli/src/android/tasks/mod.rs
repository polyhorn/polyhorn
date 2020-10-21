//! Tasks and context specific for building and running on Android.

use std::collections::HashMap;
use std::path::PathBuf;

mod build_runtime_library;
mod find_android_studio;
mod generate_source_tree;
mod install;
mod link_native_libraries;
mod run;

pub use build_runtime_library::BuildRuntimeLibrary;
pub use find_android_studio::FindAndroidStudio;
pub use generate_source_tree::GenerateSourceTree;
pub use install::Install;
pub use link_native_libraries::LinkNativeLibraries;
pub use run::Run;

pub use crate::core::tasks::InstallTarget;

use crate::core::{Manager, Task};
use crate::Config;

/// Context that gets passed through each task.
#[derive(Debug)]
pub struct AndroidContext {
    /// The platform-independent configuration that gets passed to each command.
    pub config: Config,

    /// The Java home that is resolved after the `FindAndroidStudio` task.
    pub java_home: Option<PathBuf>,

    /// The Android SDK root that is resolved after the `FindAndroidStudio`
    /// task.
    pub android_sdk_root: Option<PathBuf>,

    /// A map between Android ABIs and the path to the product for each.
    pub products: HashMap<String, PathBuf>,
}

/// Represents one of the Android-specific tasks.
pub enum AndroidTask {
    /// This tasks builds the runtime library for the given target and with the
    /// given profile.
    BuildRuntimeLibrary(BuildRuntimeLibrary),

    /// This task generates a new source tree based on the template that ships
    /// with `polyhorn-cli`.
    GenerateSourceTree(GenerateSourceTree),

    /// This task attempts to locate Android Studio and uses the result to
    /// locate the Android SDK and Java home. The Android Studio itself isn't
    /// used, but it's the easiest way of obtaining a working installation of
    /// the SDK, NDK and Java that are all compatible.
    FindAndroidStudio(FindAndroidStudio),

    /// This task invokes Gradle to install a debug-build on the user's device
    /// or emulator.
    Install(Install),

    /// This task installs a target with a given name using rustup, if
    /// necessary.
    InstallTarget(InstallTarget),

    /// This task copies all products from the `BuildRuntimeLibrary` task into
    /// the `jniLibs` folder of the Android source tree.
    LinkNativeLibraries(LinkNativeLibraries),

    /// This task launches the newly installed Polyhorn-powered app on a user's
    /// device or emulator.
    Run(Run),
}

/// Represents an error that is returned by one of the Android-specific tasks.
#[derive(Debug)]
pub enum AndroidError {
    /// Returned by tasks that have not yet been implemented for a specific host
    /// operating system.
    UnsupportedHostOS(&'static str),

    /// Returned by the `BuildRuntimeLibrary` task when Cargo fails to build the
    /// runtime library (most likely because of an error in user code, e.g.
    /// syntax error).
    CompilationFailure,

    /// Returned when platform-specific logic for locating Android Studio is
    /// implemented, but couldn't find it at the expected path (which is given
    /// as an argument).
    AndroidStudioNotFound(PathBuf),

    /// Returned when platform-specific logic for locating Java is implemented,
    /// but couldn't find it at the expected path (which is given as an
    /// argument).
    JavaNotFound(PathBuf),

    /// Returned when platform-specific logic for locating Android SDK is
    /// implemented, but couldn't find it at the expected path (which is given
    /// as an argument).
    AndroidSDKNotFound(PathBuf),

    /// Returned when platform-specific logic for locating Android NDK is
    /// implemented, but couldn't find it at the expected path (which is given
    /// as an argument).
    AndroidNDKNotFound(PathBuf),

    /// Returned by tasks when an io error occurs.
    IO(std::io::Error),
}

impl From<std::io::Error> for AndroidError {
    fn from(error: std::io::Error) -> Self {
        AndroidError::IO(error)
    }
}

impl Task for AndroidTask {
    type Context = AndroidContext;
    type Error = AndroidError;

    fn verb(&self) -> &str {
        match self {
            AndroidTask::BuildRuntimeLibrary(task) => task.verb(),
            AndroidTask::FindAndroidStudio(task) => task.verb(),
            AndroidTask::GenerateSourceTree(task) => task.verb(),
            AndroidTask::LinkNativeLibraries(task) => task.verb(),
            AndroidTask::Install(task) => task.verb(),
            AndroidTask::InstallTarget(task) => task.verb(),
            AndroidTask::Run(task) => task.verb(),
        }
    }

    fn message(&self) -> &str {
        match self {
            AndroidTask::BuildRuntimeLibrary(task) => task.message(),
            AndroidTask::FindAndroidStudio(task) => task.message(),
            AndroidTask::GenerateSourceTree(task) => task.message(),
            AndroidTask::LinkNativeLibraries(task) => task.message(),
            AndroidTask::Install(task) => task.message(),
            AndroidTask::InstallTarget(task) => task.message(),
            AndroidTask::Run(task) => task.message(),
        }
    }

    fn detail(&self) -> &str {
        match self {
            AndroidTask::BuildRuntimeLibrary(task) => task.detail(),
            AndroidTask::FindAndroidStudio(task) => task.detail(),
            AndroidTask::GenerateSourceTree(task) => task.detail(),
            AndroidTask::LinkNativeLibraries(task) => task.detail(),
            AndroidTask::Install(task) => task.detail(),
            AndroidTask::InstallTarget(task) => task.detail(),
            AndroidTask::Run(task) => task.detail(),
        }
    }

    fn run(
        &self,
        context: Self::Context,
        manager: &mut Manager,
    ) -> Result<Self::Context, Self::Error> {
        match self {
            AndroidTask::BuildRuntimeLibrary(task) => task.run(context, manager),
            AndroidTask::FindAndroidStudio(task) => task.run(context, manager),
            AndroidTask::GenerateSourceTree(task) => task.run(context, manager),
            AndroidTask::LinkNativeLibraries(task) => task.run(context, manager),
            AndroidTask::Install(task) => task.run(context, manager),
            AndroidTask::InstallTarget(task) => task
                .run((), manager)
                .map_err(|error| AndroidError::IO(error))
                .map(|_| context),
            AndroidTask::Run(task) => task.run(context, manager),
        }
    }
}
