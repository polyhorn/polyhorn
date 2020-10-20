use crate::assets::ImageSource;
use crate::styles::ImageViewStyle;

/// Renders an image source to the screen.
#[derive(Default)]
pub struct Image {
    /// Controls the appearance and layout of an Image.
    pub style: ImageViewStyle,

    /// The source of the image that is rendered to the screen.
    pub source: ImageSource,
}
