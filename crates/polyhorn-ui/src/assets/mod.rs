//! Static-typed assets that correspond to files in the `assets/` folder of a
//! Polyhorn package.

mod image;
mod image_source;

pub use image::ImageAsset;
pub use image_source::ImageSource;

/// Trait that is implemented by all static-typed asset in the `assets/` folder
/// of a package.
pub trait Asset {
    /// This function returns the name of the package that an asset originates
    /// from. Different packages are allowed to have assets with an identical
    /// name. Conversely, packages cannot access assets that originate from a
    /// different package. If you want to know the name of the package that an
    /// asset originated from, you can use this function.
    fn package(&self) -> &str;

    /// This function returns the name of this asset (without any file
    /// extension). Assets must be unique regardless of their suffix /
    /// extension. For multi-DPI assets, the name does never include the DPI
    /// suffix. The `asset!(...)` macro does not accept providing a name that
    /// includes a DPI suffix anyway.
    fn name(&self) -> &str;
}
