use std::fs::{create_dir_all, read_dir};

use super::{AndroidContext, AndroidError};
use crate::android::Target;
use crate::core::{CargoBuild, Manager, Task};

/// This tasks builds the runtime library for the given target and with the
/// given profile.
pub struct BuildRuntimeLibrary {
    /// The target ABI to compile for.
    pub target: Target<'static>,

    /// The profile to pass to Cargo, e.g. `debug` or `release`.
    pub profile: &'static str,
}

impl BuildRuntimeLibrary {
    fn setup_env(&self, context: &AndroidContext) -> Result<(), AndroidError> {
        // We start by locating the toolchain within ndk-bundle.
        let mut toolchain = context.android_sdk_root.clone().unwrap();
        toolchain.push("ndk-bundle/toolchains/llvm/prebuilt");

        let toolchain = match read_dir(&toolchain) {
            Ok(mut dir) => match dir.next() {
                Some(Ok(entry)) => entry.path(),
                _ => return Err(AndroidError::AndroidNDKNotFound(toolchain)),
            },
            Err(_) => return Err(AndroidError::AndroidNDKNotFound(toolchain)),
        };

        // And we ask downstream crates that use `polyhorn-build-android` to
        // store the jars that bundle their "native code" into the `lib` folder
        // of our Android source tree.
        let mut polyhorn_jar_dir = context.config.target_dir.clone();
        polyhorn_jar_dir.push("polyhorn-android/app/libs");

        let _ = create_dir_all(&polyhorn_jar_dir);

        if let Some(android_sdk_root) = context.android_sdk_root.as_ref() {
            std::env::set_var("ANDROID_SDK_ROOT", android_sdk_root);
        }

        if let Some(java_home) = context.java_home.as_ref() {
            std::env::set_var("JAVA_HOME", java_home);
        }

        let mut sysroot = toolchain.to_path_buf();
        sysroot.push("sysroot");

        let mut toolchain = toolchain.to_path_buf();
        toolchain.push("bin");

        let mut ar = toolchain.to_path_buf();
        ar.push(&self.target.ar);

        let mut cc = toolchain.to_path_buf();
        cc.push(&self.target.cc);

        let mut cxx = toolchain.to_path_buf();
        cxx.push(&self.target.cxx);

        let mut linker = toolchain.to_path_buf();
        linker.push(&self.target.linker);

        std::env::set_var("TARGET_AR", ar);
        std::env::set_var("TARGET_CC", cc);
        std::env::set_var("TARGET_CXX", cxx);
        std::env::set_var("POLYHORN_JAR_DIR", polyhorn_jar_dir);
        std::env::set_var(
            "CARGO_TARGET_".to_owned()
                + &self.target.llvm_triple.replace("-", "_").to_uppercase()
                + "_LINKER",
            linker,
        );

        std::env::set_var(
            "BINDGEN_EXTRA_CLANG_ARGS",
            format!("--sysroot={}", sysroot.to_str().unwrap()),
        );

        std::env::set_var(
            format!(
                "CARGO_TARGET_{}_RUSTFLAGS",
                self.target.llvm_triple.to_uppercase().replace("-", "_")
            ),
            "-Clink-arg=-lc++_static -Clink-arg=-lc++abi -Clink-arg=-fuse-ld=lld",
        );

        Ok(())
    }
}

impl Task for BuildRuntimeLibrary {
    type Context = AndroidContext;
    type Error = AndroidError;

    fn verb(&self) -> &str {
        "Building"
    }

    fn message(&self) -> &str {
        "runtime library"
    }

    fn detail(&self) -> &str {
        "for Android"
    }

    fn run(
        &self,
        mut context: AndroidContext,
        _manager: &mut Manager,
    ) -> Result<AndroidContext, AndroidError> {
        eprintln!("");

        self.setup_env(&context)?;

        let name = CargoBuild::new(&context.config.manifest_dir.join("Cargo.toml"))
            .crate_type("cdylib")
            .release(self.profile == "release")
            .target(self.target.llvm_triple)
            .build()?;

        context.products.insert(
            self.target.abi.to_owned(),
            context.config.manifest_dir.join(format!(
                "target/{}/{}/lib{}.so",
                self.target.llvm_triple, self.profile, name
            )),
        );

        Ok(context)
    }
}
