use super::{Raw, UIColor, UIImage, UIView};
use objc::runtime::*;
use objc::*;

pub struct UIImageView {
    pub(super) object: *mut Object,
}

impl UIImageView {
    pub fn new() -> UIImageView {
        unsafe {
            let object: *mut Object = msg_send![class!(PLYImageView), alloc];
            let _: () = msg_send![object, init];
            UIImageView { object }
        }
    }

    pub fn set_image(&mut self, image: &UIImage) {
        unsafe {
            let _: () = msg_send![self.object, setImage: image.as_raw()];
        }
    }

    pub fn set_tint_color(&mut self, color: &UIColor) {
        unsafe {
            let _: () = msg_send![self.object, setTintColor: color.as_raw()];
        }
    }

    pub fn to_view(&self) -> UIView {
        unsafe { UIView::from_raw_retain(self.object) }
    }
}

impl Raw for UIImageView {
    unsafe fn from_raw(object: *mut objc::runtime::Object) -> Self {
        UIImageView { object }
    }

    unsafe fn as_raw(&self) -> *mut objc::runtime::Object {
        self.object
    }
}

impl Clone for UIImageView {
    fn clone(&self) -> Self {
        unsafe { Self::from_raw_retain(self.as_raw()) }
    }
}

impl Drop for UIImageView {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
