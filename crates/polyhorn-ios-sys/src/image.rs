use objc::runtime::*;
use objc::*;

use super::foundation::NSString;
use super::{CGSize, Raw};

pub struct UIImage {
    object: *mut Object,
}

unsafe impl Send for UIImage {}
unsafe impl Sync for UIImage {}

impl UIImage {
    pub fn with_name(name: &str) -> Option<UIImage> {
        unsafe {
            let name = NSString::from(name);
            let object: *mut Object = msg_send![class!(UIImage), imageNamed: name.as_raw()];

            if object.is_null() {
                None
            } else {
                let object: *mut Object = msg_send![object, imageWithRenderingMode: 2 as usize];
                Some(UIImage {
                    object: objc_retain(object),
                })
            }
        }
    }

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
