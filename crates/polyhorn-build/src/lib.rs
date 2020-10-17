use std::env;

pub fn build() {
    println!("cargo:rerun-if-changed=build.rs");

    let target = env::var_os("CARGO_CFG_TARGET_OS");

    match target.as_ref().and_then(|target| target.to_str()) {
        Some("ios") => polyhorn_build_ios::build(),
        Some("android") => polyhorn_build_android::build(),
        _ => {}
    }
}
