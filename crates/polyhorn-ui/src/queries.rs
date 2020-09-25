//! Queries form a uniform interface to access properties of a resource. These
//! traits are implemented by the concrete asset types (e.g. FontAsset and
//! ImageAsset) and can be implemented by any other type that can also answer
//! these queries.

/// Implemented by all font-like types (including FontAsset).
pub trait FontQuery {
    /// Returns the PostScript name of this font. This is the name that should
    /// be used when assigning the font to a text style using
    /// `FontFamily::Named(...)`.
    fn post_script_name(&self) -> &'static str;
}

/// Implemented by all image-like types (including ImageAsset).
pub trait ImageQuery {
    /// This function should return the width of an image.
    fn width(&self) -> f32;

    /// This function should return the height of an image.
    fn height(&self) -> f32;
}
