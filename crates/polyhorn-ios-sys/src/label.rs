use objc::runtime::*;
use objc::*;

use super::{Raw, UIView};
use crate::foundation::NSAttributedString;

pub struct UILabel {
    pub(super) object: *mut Object,
}

impl UILabel {
    pub fn new() -> UILabel {
        unsafe {
            let object: *mut Object = msg_send![class!(PLYLabel), alloc];
            let _: () = msg_send![object, init];
            UILabel { object }
        }
    }

    pub fn set_attributed_text(&mut self, string: &NSAttributedString) {
        unsafe {
            let _: () = msg_send![self.object, setAttributedText: string.as_raw()];
        }
    }

    pub fn to_view(&self) -> UIView {
        unsafe { UIView::from_raw_retain(self.object) }
    }
}

impl Raw for UILabel {
    unsafe fn from_raw(object: *mut objc::runtime::Object) -> Self {
        UILabel { object }
    }

    unsafe fn as_raw(&self) -> *mut objc::runtime::Object {
        self.object
    }
}

impl Clone for UILabel {
    fn clone(&self) -> Self {
        unsafe { Self::from_raw(msg_send![self.as_raw(), retain]) }
    }
}

impl Drop for UILabel {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.object, release];
        }
    }
}
