use strum_macros::EnumString;

use super::ViewStyle;
use crate::color::Color;

/// Defines the method for fitting objects that do not match the dimensions of
/// their container.
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumString)]
pub enum ObjectFit {
    /// Fills the container without respecting the aspect ratio of the object if
    /// the dimensions of the container isn't a multiple of the dimensions of
    /// the object.
    #[strum(serialize = "fill")]
    Fill,

    /// Applies no scaling to the object.
    #[strum(serialize = "none")]
    None,

    /// Scales the object so that it is completely visible while maintaining its
    /// aspect ratio. If after scaling, one of the dimensions is smaller than
    /// the corresponding dimension of the container, the object is centered
    /// within its container.
    #[strum(serialize = "contain")]
    Contain,

    /// Scales the object so that it completely covers its container, while
    /// maintaining its aspect ratio. If after scaling, one of the dimensions is
    /// larger than the corresponding dimension of the container, the object is
    /// centered within its container and the invisible area is split evenly
    /// across both ends of the relevant dimension.
    #[strum(serialize = "cover")]
    Cover,
}

impl Default for ObjectFit {
    fn default() -> Self {
        ObjectFit::Fill
    }
}

/// Controls the appearance of an Image.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct ImageStyle {
    /// Controls the method for fitting images that do not match the dimensions
    /// of their container.
    pub object_fit: ObjectFit,

    /// If present, controls the color that this image is rendered in. Only the
    /// alpha channel of the original image is kept: all other channels are
    /// replaced by the given tint color. If this tint color exists in a
    /// different color space than the original image, the resulting image is
    /// drawn using the color space of the tint color.
    pub tint_color: Option<Color>,
}

/// This is a union style of the Image and View styles.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct ImageViewStyle {
    /// This style contains the properties that are only applicable to Images.
    pub image: ImageStyle,

    /// This style contains the properties that are only applicable to Views.
    pub view: ViewStyle,
}

impl From<ImageStyle> for ImageViewStyle {
    fn from(style: ImageStyle) -> Self {
        ImageViewStyle {
            image: style,
            ..Default::default()
        }
    }
}

impl From<ViewStyle> for ImageViewStyle {
    fn from(style: ViewStyle) -> Self {
        ImageViewStyle {
            view: style,
            ..Default::default()
        }
    }
}
