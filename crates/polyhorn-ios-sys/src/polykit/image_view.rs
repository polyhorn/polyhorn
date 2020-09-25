use objc::runtime::*;
use objc::*;

use super::PLYView;
use crate::uikit::{UIColor, UIImage};
use crate::Raw;

/// An object that displays a single image or a sequence of animated images in
/// your interface.
pub struct PLYImageView {
    pub(super) object: *mut Object,
}

impl PLYImageView {
    /// Initializes a newly allocated image view.
    pub fn new() -> PLYImageView {
        unsafe {
            let mut object: *mut Object = msg_send![class!(PLYImageView), alloc];
            object = msg_send![object, init];
            PLYImageView { object }
        }
    }

    /// Sets the image displayed in the image view.
    pub fn set_image(&mut self, image: &UIImage) {
        unsafe {
            let _: () = msg_send![self.object, setImage: image.as_raw()];
        }
    }

    /// Tints the image displayed in the image view with the given color.
    pub fn set_tint_color(&mut self, color: &UIColor) {
        unsafe {
            let _: () = msg_send![self.object, setTintColor: color.as_raw()];
        }
    }

    /// Upcasts this view to a UIView.
    pub fn to_view(&self) -> PLYView {
        unsafe { PLYView::from_raw_retain(self.object) }
    }
}

impl Raw for PLYImageView {
    unsafe fn from_raw(object: *mut Object) -> Self {
        PLYImageView { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Drop for PLYImageView {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
