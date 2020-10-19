use ansi_term::Colour::Red;
use cargo::core::compiler::{CompileKind, CompileMode, CompileTarget, CrateType};
use cargo::core::manifest::TargetKind;
use cargo::core::Workspace;
use cargo::ops::{compile, CompileOptions};
use cargo::util::interning::InternedString;
use cargo::util::Config;
use std::fs::{create_dir_all, read_dir};
use std::path::{Path, PathBuf};

use super::{AndroidContext, AndroidError};
use crate::android::Target;
use crate::core::{Manager, Task};

/// This tasks builds the runtime library for the given target and with the
/// given profile.
pub struct BuildRuntimeLibrary {
    /// The target ABI to compile for.
    pub target: Target<'static>,

    /// The profile to pass to Cargo, e.g. `debug` or `release`.
    pub profile: &'static str,
}

impl BuildRuntimeLibrary {
    /// Invokes Cargo to build the runtime library from the given manifest with
    /// the given toolchain. The polyhorn jar dir will store the products of
    /// `polyhorn-build-android-jar`, i.e. all bundles of Java code provided by
    /// the dependencies of the library we're building.
    pub fn build(
        &self,
        toolchain: &Path,
        manifest_path: &Path,
        polyhorn_jar_dir: &Path,
    ) -> Result<PathBuf, AndroidError> {
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

        let mut config = Config::default().unwrap();
        config
            .configure(0, false, None, false, false, false, &None, &[], &[])
            .unwrap();

        let mut workspace = Workspace::new(manifest_path, &config).unwrap();

        let targets = workspace
            .current_mut()
            .unwrap()
            .manifest_mut()
            .targets_mut();

        assert_eq!(targets.len(), 1);
        assert!(matches!(targets[0].kind(), TargetKind::Lib(_)));
        targets[0].set_kind(TargetKind::Lib(vec![CrateType::Cdylib]));

        let mut options = CompileOptions::new(&config, CompileMode::Build).unwrap();
        options.target_rustc_args = Some(vec![
            // We statically link to libc++.
            "-Clink-arg=-lc++_static".to_owned(),
            "-Clink-arg=-lc++abi".to_owned(),
            "-Clink-arg=-fuse-ld=lld".to_owned(),
        ]);
        options.build_config.requested_profile = InternedString::new(self.profile);
        options.build_config.requested_kinds = vec![CompileKind::Target(
            CompileTarget::new(self.target.llvm_triple).unwrap(),
        )];

        match compile(&workspace, &options) {
            Ok(mut compilation) => Ok(compilation.cdylibs.remove(0).1),
            Err(error) => {
                eprintln!("{}: {:?}", Red.bold().paint("error"), error);
                Err(AndroidError::CompilationFailure)
            }
        }
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

        // Then we locate the Cargo manifest.
        let mut manifest_path = context.config.manifest_dir.clone();
        manifest_path.push("Cargo.toml");

        // And we ask downstream crates that use `polyhorn-build-android` to
        // store the jars that bundle their "native code" into the `lib` folder
        // of our Android source tree.
        let mut polyhorn_jar_dir = context.config.manifest_dir.clone();
        polyhorn_jar_dir.push("target/polyhorn-android/app/libs");

        let _ = create_dir_all(&polyhorn_jar_dir);

        // Cargo wants to start at a new line.
        eprintln!("");

        if let Some(android_sdk_root) = context.android_sdk_root.as_ref() {
            std::env::set_var("ANDROID_SDK_ROOT", android_sdk_root);
        }

        if let Some(java_home) = context.java_home.as_ref() {
            std::env::set_var("JAVA_HOME", java_home);
        }

        let result = self.build(&toolchain, &manifest_path, &polyhorn_jar_dir);

        match result {
            Ok(path) => {
                context.products.insert(self.target.abi.to_owned(), path);
                Ok(context)
            }
            Err(error) => Err(error),
        }
    }
}
