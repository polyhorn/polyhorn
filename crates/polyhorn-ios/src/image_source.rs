use polyhorn_ios_sys::UIImage;
use std::fmt::{Debug, Formatter, Result};

#[derive(Clone)]
pub struct ImageSource(UIImage);

impl ImageSource {
    pub fn with_name(name: &str) -> Option<ImageSource> {
        UIImage::with_name(name).map(|image| ImageSource(image))
    }

    pub fn width(&self) -> usize {
        self.0.size().width.ceil() as usize
    }

    pub fn height(&self) -> usize {
        self.0.size().height.ceil() as usize
    }
}

impl Debug for ImageSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let size = self.0.size();

        f.debug_struct("ImageSource")
            .field("width", &size.width)
            .field("height", &size.height)
            .finish()
    }
}

impl From<ImageSource> for UIImage {
    fn from(value: ImageSource) -> Self {
        value.0
    }
}
