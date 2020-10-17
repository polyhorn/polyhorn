use std::fs::create_dir_all;

use super::{AndroidContext, AndroidError};
use crate::android::Target;
use crate::core::{Manager, Task};

/// This task copies all products from the `BuildRuntimeLibrary` task into the
/// `jniLibs` folder of the Android source tree.
pub struct LinkNativeLibraries;

impl Task for LinkNativeLibraries {
    type Context = AndroidContext;
    type Error = AndroidError;

    fn verb(&self) -> &str {
        "Linking"
    }

    fn message(&self) -> &str {
        "native libraries"
    }

    fn detail(&self) -> &str {
        "for Android"
    }

    fn run(
        &self,
        context: AndroidContext,
        _manager: &mut Manager,
    ) -> Result<AndroidContext, AndroidError> {
        let mut destination_path = context.config.target_dir.clone();
        destination_path.push("polyhorn-android/app/src/main/jniLibs");

        for target in Target::all().iter() {
            let source_path = match context.products.get(target.abi) {
                Some(path) => path,
                _ => continue,
            };

            let mut destination_path = destination_path.clone();
            destination_path.push(target.abi);

            let _ = create_dir_all(&destination_path);

            destination_path.push(format!("lib{}.so", context.config.spec.app.android.library));
            std::fs::copy(source_path, destination_path).unwrap();
        }

        Ok(context)
    }
}
