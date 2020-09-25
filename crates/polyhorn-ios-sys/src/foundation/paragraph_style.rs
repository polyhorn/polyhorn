use objc::runtime::*;
use objc::*;

use crate::Raw;

#[repr(usize)]
pub enum NSTextAlignment {
    Left = 0,

    // This is not even a joke:
    #[cfg(target_os = "ios")]
    Center = 1,

    #[cfg(not(target_os = "ios"))]
    Right = 1,

    #[cfg(target_os = "ios")]
    Right = 2,

    #[cfg(not(target_os = "ios"))]
    Center = 2,
}

pub struct NSParagraphStyle {
    object: *mut Object,
}

pub struct NSMutableParagraphStyle {
    object: *mut Object,
}

impl NSMutableParagraphStyle {
    pub fn new() -> NSMutableParagraphStyle {
        unsafe {
            let mut object: *mut Object = msg_send![class!(NSMutableParagraphStyle), alloc];
            object = msg_send![object, init];
            NSMutableParagraphStyle { object }
        }
    }

    pub fn set_alignment(&mut self, alignment: NSTextAlignment) {
        unsafe {
            let _: () = msg_send![self.object, setAlignment: alignment];
        }
    }
}

impl Raw for NSMutableParagraphStyle {
    unsafe fn from_raw(object: *mut objc::runtime::Object) -> Self {
        NSMutableParagraphStyle { object }
    }

    unsafe fn as_raw(&self) -> *mut objc::runtime::Object {
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
    unsafe fn from_raw(object: *mut objc::runtime::Object) -> Self {
        NSParagraphStyle { object }
    }

    unsafe fn as_raw(&self) -> *mut objc::runtime::Object {
        self.object
    }
}

impl Clone for NSParagraphStyle {
    fn clone(&self) -> Self {
        unsafe { Self::from_raw(msg_send![self.as_raw(), retain]) }
    }
}

impl Drop for NSParagraphStyle {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.object, release];
        }
    }
}
