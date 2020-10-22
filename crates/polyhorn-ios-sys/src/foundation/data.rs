use objc::runtime::*;
use objc::*;

use crate::Raw;

/// A static byte buffer in memory.
pub struct NSData {
    object: *mut Object,
}

impl From<&[u8]> for NSData {
    fn from(value: &[u8]) -> Self {
        unsafe {
            let mut object: *mut Object = msg_send![class!(NSData), alloc];
            object = msg_send![object, initWithBytes: value.as_ptr()
                                              length: value.len() as usize];
            NSData::from_raw(object)
        }
    }
}

impl Raw for NSData {
    unsafe fn from_raw(object: *mut Object) -> Self {
        NSData { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Drop for NSData {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
