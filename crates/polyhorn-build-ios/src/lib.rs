use std::fs::read_dir;

pub fn build() {
    let mut build = cc::Build::new();
    build.flag("-isysroot/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator.sdk");
    build.flag("-fobjc-arc");
    build.flag("-O3");

    let mut empty = true;

    if let Ok(dir) = read_dir("ios") {
        for entry in dir {
            if let Ok(entry) = entry {
                println!(
                    "cargo:rerun-if-changed={}",
                    entry.path().to_str().unwrap_or_default()
                );

                if entry
                    .path()
                    .extension()
                    .map(|ext| ext == "m")
                    .unwrap_or_default()
                {
                    build.file(entry.path());
                    empty = false;
                }
            }
        }
    }

    if empty {
        return;
    }

    build.compile("libpolyhorn_build_ios.a");
}
