use std::process::{Command, Stdio};

use super::{AndroidContext, AndroidError};
use crate::core::{Manager, Task};

/// This task launches the newly installed Polyhorn-powered app on a user's
/// device or emulator.
pub struct Run;

impl Task for Run {
    type Context = AndroidContext;
    type Error = AndroidError;

    fn verb(&self) -> &str {
        "Launching"
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
        let android_sdk_root = context.android_sdk_root.as_ref().unwrap();

        let mut working_directory = destination_path.clone();
        working_directory.push("polyhorn-android");

        let mut adb = android_sdk_root.clone();
        adb.push("platform-tools/adb");

        let mut command = Command::new(adb);
        command.current_dir(working_directory);
        command.args(&["shell", "am", "start", "-n"]);
        command.arg(format!(
            "{}/{}.MainActivity",
            context.config.spec.app.android.package, context.config.spec.app.android.package
        ));
        command.stdout(Stdio::piped());
        let output = command.spawn();

        output.unwrap().wait().unwrap();

        Ok(context)
    }
}
