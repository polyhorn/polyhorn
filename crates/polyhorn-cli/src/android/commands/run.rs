use ansi_term::Colour::Red;
use std::collections::HashMap;
use std::path::Path;

use crate::android::tasks::{self, AndroidContext, AndroidTask};
use crate::android::Target;
use crate::core::Executioner;
use crate::Config;

/// This is the implementation of `polyhorn run android`.
pub fn run(config: Config) {
    let result = Executioner::execute(
        &[
            AndroidTask::InstallTarget(tasks::InstallTarget(Target::armeabi_v7a().llvm_triple)),
            AndroidTask::FindAndroidStudio(tasks::FindAndroidStudio),
            AndroidTask::GenerateSourceTree(tasks::GenerateSourceTree),
            AndroidTask::BuildRuntimeLibrary(tasks::BuildRuntimeLibrary {
                target: Target::armeabi_v7a(),
                profile: "debug",
            }),
            AndroidTask::LinkNativeLibraries(tasks::LinkNativeLibraries),
            AndroidTask::Install(tasks::Install),
            AndroidTask::Run(tasks::Run),
        ],
        AndroidContext {
            config,
            java_home: std::env::var("JAVA_HOME")
                .ok()
                .map(|path| Path::new(&path).to_path_buf()),
            android_sdk_root: std::env::var("ANDROID_SDK_ROOT")
                .ok()
                .map(|path| Path::new(&path).to_path_buf()),
            products: HashMap::new(),
        },
    );

    match result {
        Ok(_) => {}
        Err(error) => {
            eprintln!("{}: {:?}", Red.bold().paint("error"), error);
            std::process::exit(1);
        }
    }
}
