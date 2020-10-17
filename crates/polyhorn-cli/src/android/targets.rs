/// Represents a Android ABI target that is used to build for an Android device
/// with a specific CPU and instruction set.
pub struct Target<'a> {
    /// This is the name for the ABI that Android uses internally (e.g. in
    /// `jniLibs`).
    pub abi: &'a str,

    /// This is the name of the target that llvm/rustc uses.
    pub llvm_triple: &'a str,

    /// This is the name of the archiver utility shipped with the NDK.
    pub ar: &'a str,

    /// This is the name of the C compiler shipped with the NDK.
    pub cc: &'a str,

    /// This is the name of the C++ compiler shipped with the NDK.
    pub cxx: &'a str,

    /// This is the name of the linker shipped with the NDK.
    pub linker: &'a str,
}

impl<'a> Target<'a> {
    /// Returns all available Android targets.
    pub const fn all() -> &'static [Target<'static>] {
        &[
            Target::armeabi_v7a(),
            Target::arm64_v8a(),
            Target::x86(),
            Target::x86_64(),
        ]
    }

    /// Returns a target configuration for the `armeabi-v7a` ABI.
    pub const fn armeabi_v7a() -> Target<'static> {
        Target {
            abi: "armeabi-v7a",
            llvm_triple: "armv7-linux-androideabi",
            ar: "arm-linux-androideabi-ar",
            cc: "armv7a-linux-androideabi16-clang",
            cxx: "armv7a-linux-androideabi16-clang++",
            linker: "armv7a-linux-androideabi16-clang",
        }
    }

    /// Returns a target configuration for the `arm64-v8a` ABI (also known as
    /// `aarch64`). This target has been available since API level 21 (the first
    /// version of Android to support 64-bit architectures).
    pub const fn arm64_v8a() -> Target<'static> {
        Target {
            abi: "arm64-v8a",
            llvm_triple: "aarch64-linux-android",
            ar: "aarch64-linux-android-ar",
            cc: "aarch64-linux-android21-clang",
            cxx: "aarch64-linux-android21-clang++",
            linker: "aarch64-linux-android21-clang",
        }
    }

    /// Returns a target configuration for the `x86` ABI (also known as `i686`).
    pub const fn x86() -> Target<'static> {
        Target {
            abi: "x86",
            llvm_triple: "i686-linux-android",
            ar: "i686-linux-android-ar",
            cc: "i686-linux-android16-clang",
            cxx: "i686-linux-android16-clang++",
            linker: "i686-linux-android16-clang",
        }
    }

    /// Returns a target configuration for the `x86_64` ABI. This target has
    /// been available since API level 21 (the first version of Android to
    /// support 64-bit architectures).
    pub const fn x86_64() -> Target<'static> {
        Target {
            abi: "x86_64",
            llvm_triple: "x86_64-linux-android",
            ar: "x86_64-linux-android-ar",
            cc: "x86_64-linux-android21-clang",
            cxx: "x86_64-linux-android21-clang++",
            linker: "x86_64-linux-android21-clang",
        }
    }
}
