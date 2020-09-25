use super::Raw;
use fsize::fsize;
use objc::runtime::*;
use objc::*;

pub struct UIFont {
    object: *mut Object,
}

impl UIFont {
    pub fn system_font_of_size(size: fsize) -> UIFont {
        unsafe { UIFont::from_raw(msg_send![class!(UIFont), systemFontOfSize: size]) }
    }

    pub fn bold_system_font_of_size(size: fsize) -> UIFont {
        unsafe { UIFont::from_raw(msg_send![class!(UIFont), boldSystemFontOfSize: size]) }
    }
}

/// UIFont is safe to send to other threads according to Apple's documentation.
unsafe impl Send for UIFont {}

/// UIFont is immutable and can be accessed from multiple threads at the same
/// time according to Apple's documentation.
unsafe impl Sync for UIFont {}

impl Raw for UIFont {
    unsafe fn from_raw(object: *mut objc::runtime::Object) -> Self {
        UIFont { object }
    }

    unsafe fn as_raw(&self) -> *mut objc::runtime::Object {
        self.object
    }
}

impl Clone for UIFont {
    fn clone(&self) -> Self {
        unsafe {
            UIFont {
                object: msg_send![self.object, retain],
            }
        }
    }
}

impl Drop for UIFont {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.object, release];
        }
    }
}
