use objc::runtime::*;
use objc::*;

use super::{Raw, UIView};
use crate::foundation::NSAttributedString;

pub struct UITextInputView {
    pub(super) object: *mut Object,
}

impl UITextInputView {
    pub fn new() -> UITextInputView {
        unsafe {
            let object: *mut Object = msg_send![class!(PLYTextInputView), alloc];
            let _: () = msg_send![object, init];
            UITextInputView { object }
        }
    }

    pub fn set_attributed_placeholder(&mut self, placeholder: &NSAttributedString) {
        unsafe {
            let _: () = msg_send![self.object, setAttributedPlaceholder: placeholder.as_raw()];
        }
    }

    pub fn to_view(&self) -> UIView {
        unsafe { UIView::from_raw_retain(self.object) }
    }
}

impl Raw for UITextInputView {
    unsafe fn from_raw(object: *mut objc::runtime::Object) -> Self {
        UITextInputView { object }
    }

    unsafe fn as_raw(&self) -> *mut objc::runtime::Object {
        self.object
    }
}

impl Clone for UITextInputView {
    fn clone(&self) -> Self {
        unsafe { Self::from_raw_retain(self.as_raw()) }
    }
}

impl Drop for UITextInputView {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
