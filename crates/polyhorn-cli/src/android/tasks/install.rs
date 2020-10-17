use std::process::{Command, Stdio};

use super::{AndroidContext, AndroidError};
use crate::core::{Manager, Task};

/// This task invokes Gradle to install a debug-build on the user's device or
/// emulator.
pub struct Install;

impl Task for Install {
    type Context = AndroidContext;
    type Error = AndroidError;

    fn verb(&self) -> &str {
        "Installing"
    }

    fn message(&self) -> &str {
        "Android App"
    }

    fn detail(&self) -> &str {
        ""
    }

    fn run(
        &self,
        context: AndroidContext,
        _manager: &mut Manager,
    ) -> Result<AndroidContext, AndroidError> {
        let destination_path = context.config.target_dir.clone();
        let java_home = context.java_home.as_ref().unwrap();
        let android_sdk_root = context.android_sdk_root.as_ref().unwrap();

        let mut working_directory = destination_path.clone();
        working_directory.push("polyhorn-android");

        let mut gradlew = working_directory.clone();
        gradlew.push("gradlew");

        let mut command = Command::new(gradlew);
        command.current_dir(working_directory);
        command.env("ANDROID_SDK_ROOT", android_sdk_root);
        command.env("JAVA_HOME", java_home);
        command.arg("installDebug");
        command.stdout(Stdio::piped());
        let output = command.spawn();

        output.unwrap().wait().unwrap();

        Ok(context)
    }
}
