//! Types to abstract away the source of an image that is drawn to the screen.

use bytes::Bytes;

use super::ImageAsset;
use crate::geometry::Size;

/// An image source describes the source of an image that can be passed to e.g.
/// the Image component. An image source can either be a placeholder, an Asset
/// or a Bytes (3rd party crate) object.
#[derive(Clone, PartialEq)]
pub enum ImageSource {
    /// If you're loading a remote image, you might want to show a placeholder
    /// instead. The placeholder is entirely transparent (unless you give the
    /// containing component an opaque background color) but does have a size.
    /// This size is subsequently used in calculating the layout.
    ///
    /// The benefit of this is that when the image is eventually completely
    /// loaded and the component re-renders to show that image, the rest of the
    /// layout will not change because it already anticipated the correct size
    /// of the image.
    Placeholder(Size<f32>),

    /// This image source is backed by a local asset that is included in a
    /// Polyhorn package.
    Asset(ImageAsset),

    /// This image source is backed by an PNG-encoded buffer.
    Bytes(Bytes),
}

impl Default for ImageSource {
    fn default() -> Self {
        ImageSource::Placeholder(Size::default())
    }
}

impl From<ImageAsset> for ImageSource {
    fn from(asset: ImageAsset) -> Self {
        ImageSource::Asset(asset)
    }
}

impl From<Bytes> for ImageSource {
    fn from(bytes: Bytes) -> Self {
        ImageSource::Bytes(bytes)
    }
}

impl From<&'static [u8]> for ImageSource {
    fn from(bytes: &'static [u8]) -> Self {
        ImageSource::Bytes(Bytes::from_static(bytes))
    }
}
