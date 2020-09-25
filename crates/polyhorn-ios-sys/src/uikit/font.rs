use objc::runtime::*;
use objc::*;

use crate::coregraphics::CGFloat;
use crate::Raw;

/// A font object.
pub struct UIFont {
    object: *mut Object,
}

impl UIFont {
    /// Returns the font object for standard interface items in the specified
    /// size.
    pub fn system_font_of_size(size: CGFloat) -> UIFont {
        unsafe { UIFont::from_raw_retain(msg_send![class!(UIFont), systemFontOfSize: size]) }
    }

    /// Returns the font object for standard interface items in boldface type in
    /// the specified size.
    pub fn bold_system_font_of_size(size: CGFloat) -> UIFont {
        unsafe { UIFont::from_raw_retain(msg_send![class!(UIFont), boldSystemFontOfSize: size]) }
    }
}

/// UIFont is safe to send to other threads according to Apple's documentation.
unsafe impl Send for UIFont {}

/// UIFont is immutable and can be accessed from multiple threads at the same
/// time according to Apple's documentation.
unsafe impl Sync for UIFont {}

impl Raw for UIFont {
    unsafe fn from_raw(object: *mut Object) -> Self {
        UIFont { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Clone for UIFont {
    fn clone(&self) -> Self {
        unsafe { UIFont::from_raw_retain(self.as_raw()) }
    }
}

impl Drop for UIFont {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
