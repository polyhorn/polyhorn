use objc::runtime::*;
use objc::*;

use crate::coregraphics::CGSize;
use crate::foundation::{NSData, NSString};
use crate::Raw;

/// An object that manes image data in your app.
pub struct UIImage {
    object: *mut Object,
}

unsafe impl Send for UIImage {}
unsafe impl Sync for UIImage {}

impl UIImage {
    /// Creates an image object from the specified named asset.
    pub fn with_name(name: &str) -> Option<UIImage> {
        unsafe {
            let name = NSString::from(name);
            let object: *mut Object = msg_send![class!(UIImage), imageNamed: name.as_raw()];

            if object.is_null() {
                None
            } else {
                Some(UIImage {
                    object: objc_retain(object),
                })
            }
        }
    }

    /// Creates and returns an image object that uses the specified image data.
    pub fn with_data(bytes: &[u8]) -> Option<UIImage> {
        unsafe {
            let data = NSData::from(bytes);
            let object: *mut Object = msg_send![class!(UIImage), imageWithData: data.as_raw()];

            if object.is_null() {
                None
            } else {
                Some(UIImage {
                    object: objc_retain(object),
                })
            }
        }
    }

    /// Returns a copy of this image with template rendering mode.
    pub fn templated(&self) -> UIImage {
        unsafe {
            let object: *mut Object = msg_send![self.object, imageWithRenderingMode: 2 as usize];

            UIImage {
                object: objc_retain(object),
            }
        }
    }

    /// The logical dimensions, in points, for the image.
    pub fn size(&self) -> CGSize {
        unsafe { msg_send![self.object, size] }
    }
}

impl Raw for UIImage {
    unsafe fn from_raw(object: *mut Object) -> Self {
        UIImage { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Clone for UIImage {
    fn clone(&self) -> Self {
        unsafe { Self::from_raw_retain(self.as_raw()) }
    }
}

impl Drop for UIImage {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
