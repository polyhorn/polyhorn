use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use walkdir::WalkDir;

pub struct Config {
    java_home: PathBuf,
    android_jar: PathBuf,
}

pub fn build() {
    let mut empty = true;
    let mut files = vec![];

    for entry in WalkDir::new("android") {
        if let Ok(entry) = entry {
            println!(
                "cargo:rerun-if-changed={}",
                entry.path().to_str().unwrap_or_default()
            );

            if entry
                .path()
                .extension()
                .map(|ext| ext == "java")
                .unwrap_or_default()
            {
                files.push(entry.path().to_path_buf());
                empty = false;
            }
        }
    }

    if empty {
        return;
    }

    let java_home = Path::new(&std::env::var("JAVA_HOME").unwrap()).to_path_buf();
    let android_sdk_root = Path::new(&std::env::var("ANDROID_SDK_ROOT").unwrap()).to_path_buf();

    let mut android_jar = android_sdk_root.clone();
    android_jar.push("platforms/android-30/android.jar");

    let config = Config {
        java_home,
        android_jar,
    };

    let mut javac = config.java_home.clone();
    javac.push("bin/javac");

    let mut bootclasspath = config.java_home.clone();
    bootclasspath.push("jre/lib/rt.jar");

    let mut destination_dir = Path::new(&std::env::var_os("OUT_DIR").unwrap()).to_path_buf();
    destination_dir.push("android");

    let _ = create_dir_all(&destination_dir);

    let mut command = Command::new(javac);
    command.args(&["-source", "1.8", "-target", "1.8", "-bootclasspath"]);
    command.arg(&bootclasspath);
    command.arg("-classpath");
    command.arg(&config.android_jar);
    command.arg("-d");
    command.arg(&destination_dir);
    command.args(&files);
    command.stdout(Stdio::piped());
    assert!(command.spawn().unwrap().wait().unwrap().success());

    let mut jar = config.java_home.clone();
    jar.push("bin/jar");

    let mut classes_path = Path::new(&std::env::var_os("OUT_DIR").unwrap()).to_path_buf();
    classes_path.push("android");

    let mut destination_path = Path::new(&std::env::var_os("OUT_DIR").unwrap()).to_path_buf();
    destination_path.push(format!("{}.jar", std::env::var("CARGO_PKG_NAME").unwrap()));

    let mut command = Command::new(&jar);
    command.arg("cf");
    command.arg(&destination_path);
    command.arg("-C");
    command.arg(&classes_path);
    command.arg(".");
    command.stdout(Stdio::piped());
    assert!(command.spawn().unwrap().wait().unwrap().success());

    let mut jar_path = Path::new(&std::env::var_os("POLYHORN_JAR_DIR").unwrap()).to_path_buf();
    jar_path.push(format!("{}.jar", std::env::var("CARGO_PKG_NAME").unwrap()));

    std::fs::copy(destination_path, jar_path).unwrap();
}
