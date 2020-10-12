use objc::runtime::*;
use objc::*;

use crate::coregraphics::CGRect;
use crate::Raw;

pub struct PLYLayoutEvent {
    object: *mut Object,
}

impl PLYLayoutEvent {
    pub fn frame(&self) -> CGRect {
        unsafe { msg_send![self.object, frame] }
    }
}

impl Raw for PLYLayoutEvent {
    unsafe fn from_raw(object: *mut Object) -> Self {
        PLYLayoutEvent { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Drop for PLYLayoutEvent {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
