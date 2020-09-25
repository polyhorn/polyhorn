use objc::runtime::*;
use objc::*;

use crate::{IntoRaw, Raw};

/// A static, plain-text Unicode string object.
pub struct NSString {
    object: *mut Object,
}

const NS_UTF8_STRING_ENCODING: usize = 4;

impl From<&str> for NSString {
    fn from(value: &str) -> Self {
        unsafe {
            let mut object: *mut Object = msg_send![class!(NSString), alloc];
            object = msg_send![object, initWithBytes: value.as_ptr()
                                              length: value.len() as usize
                                            encoding: NS_UTF8_STRING_ENCODING];
            NSString::from_raw(object)
        }
    }
}

impl IntoRaw for &String {
    type Raw = NSString;

    fn into_raw(self) -> Self::Raw {
        self.as_str().into()
    }
}

impl Raw for NSString {
    unsafe fn from_raw(object: *mut Object) -> Self {
        NSString { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Drop for NSString {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
