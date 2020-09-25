use super::Asset;
use crate::queries::ImageQuery;

/// Static-typed asset for images.
#[derive(Copy, Clone, Debug)]
pub struct ImageAsset {
    package: &'static str,
    name: &'static str,
    width: f32,
    height: f32,
}

impl ImageAsset {
    #[doc(hidden)]
    pub fn new(package: &'static str, name: &'static str, width: f32, height: f32) -> ImageAsset {
        ImageAsset {
            package,
            name,
            width,
            height,
        }
    }
}

impl PartialEq for ImageAsset {
    fn eq(&self, other: &ImageAsset) -> bool {
        self.package == other.package && self.name == other.name
    }
}

impl Eq for ImageAsset {}

impl Asset for ImageAsset {
    fn package(&self) -> &str {
        self.package
    }

    fn name(&self) -> &str {
        self.name
    }
}

impl ImageQuery for ImageAsset {
    fn width(&self) -> f32 {
        self.width
    }

    fn height(&self) -> f32 {
        self.height
    }
}
