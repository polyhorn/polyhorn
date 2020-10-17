use std::path::Path;

use super::{AndroidContext, AndroidError};
use crate::core::{Manager, Task};

/// This task attempts to locate Android Studio and uses the result to locate
/// the Android SDK and Java home. The Android Studio itself isn't used, but
/// it's the easiest way of obtaining a working installation of the SDK, NDK and
/// Java that are all compatible.
pub struct FindAndroidStudio;

impl Task for FindAndroidStudio {
    type Context = AndroidContext;
    type Error = AndroidError;

    fn verb(&self) -> &str {
        "Configuring"
    }

    fn message(&self) -> &str {
        "Android Studio"
    }

    fn detail(&self) -> &str {
        ""
    }

    fn run(
        &self,
        mut context: AndroidContext,
        _manager: &mut Manager,
    ) -> Result<AndroidContext, AndroidError> {
        #[cfg(target_os = "macos")]
        {
            // We start by looking for Android Studio itself.
            let path = Path::new("/Applications/Android Studio.app").to_path_buf();
            let studio = match path.exists() {
                true => path,
                false => return Err(AndroidError::AndroidStudioNotFound(path)),
            };

            // Android Studio ships with a compatible release of OpenJDK 8.
            let mut java_home = studio.clone();
            java_home.push("Contents/jre/jdk/Contents/Home");

            match java_home.exists() {
                true => context.java_home = Some(java_home),
                false => return Err(AndroidError::JavaNotFound(java_home)),
            }

            // If Android Studio is installed, and the user has opted to install
            // the default components, the SDK will have been installed in
            // `~/Library/Android/sdk`.
            let android_sdk_root = Path::new(&format!(
                "{}/Library/Android/sdk",
                dirs::home_dir().unwrap().to_str().unwrap()
            ))
            .to_path_buf();

            match android_sdk_root.exists() {
                true => context.android_sdk_root = Some(android_sdk_root),
                false => return Err(AndroidError::AndroidSDKNotFound(android_sdk_root)),
            }

            return Ok(context);
        }

        // TODO: we don't have heuristics to find Android Studio on non-macOS
        // systems yet.
        #[allow(unreachable_code)]
        Err(AndroidError::UnsupportedHostOS(
            "We can't yet automatically locate Android Studio on your OS.",
        ))
    }
}
