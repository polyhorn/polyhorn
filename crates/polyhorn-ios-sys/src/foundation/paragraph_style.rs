use objc::runtime::*;
use objc::*;

use crate::Raw;

/// Constants that specify text alignment.
#[repr(usize)]
pub enum NSTextAlignment {
    /// Text is visually left aligned.
    Left = 0,

    #[cfg(target_os = "ios")]
    /// Text is visually center-aligned.
    Center = 1,

    #[cfg(not(target_os = "ios"))]
    /// Text is visually right-aligned.
    Right = 1,

    #[cfg(target_os = "ios")]
    /// Text is visually right-aligned.
    Right = 2,

    #[cfg(not(target_os = "ios"))]
    /// Text is visually center-aligned.
    Center = 2,
}

/// The paragraph or ruler attributes for an attributed string.
pub struct NSParagraphStyle {
    object: *mut Object,
}

/// An object for changing the values of the subattributes in a paragraph style
/// attribute.
pub struct NSMutableParagraphStyle {
    object: *mut Object,
}

impl NSMutableParagraphStyle {
    /// Initializes a newly allocated mutable paragraph style.
    pub fn new() -> NSMutableParagraphStyle {
        unsafe {
            let mut object: *mut Object = msg_send![class!(NSMutableParagraphStyle), alloc];
            object = msg_send![object, init];
            NSMutableParagraphStyle { object }
        }
    }

    /// Sets the text alignment of the paragraph.
    pub fn set_alignment(&mut self, alignment: NSTextAlignment) {
        unsafe {
            let _: () = msg_send![self.object, setAlignment: alignment];
        }
    }
}

impl Raw for NSMutableParagraphStyle {
    unsafe fn from_raw(object: *mut Object) -> Self {
        NSMutableParagraphStyle { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Drop for NSMutableParagraphStyle {
    fn drop(&mut self) {
        unsafe {
            objc_release(self.object);
        }
    }
}

impl Into<NSParagraphStyle> for NSMutableParagraphStyle {
    fn into(self) -> NSParagraphStyle {
        unsafe { NSParagraphStyle::from_raw_retain(self.as_raw()) }
    }
}

impl Raw for NSParagraphStyle {
    unsafe fn from_raw(object: *mut Object) -> Self {
        NSParagraphStyle { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Clone for NSParagraphStyle {
    fn clone(&self) -> Self {
        unsafe { Self::from_raw_retain(self.as_raw()) }
    }
}

impl Drop for NSParagraphStyle {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
