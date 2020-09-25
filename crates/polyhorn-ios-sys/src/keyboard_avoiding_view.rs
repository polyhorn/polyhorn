use super::{Raw, UICallback, UIView};
use crate::foundation::NSNumber;
use objc::runtime::*;
use objc::*;

pub struct UIKeyboardAvoidingView {
    pub(super) object: *mut Object,
}

impl UIKeyboardAvoidingView {
    pub fn new() -> UIKeyboardAvoidingView {
        unsafe {
            let object: *mut Object = msg_send![class!(PLYKeyboardAvoidingView), alloc];
            let _: () = msg_send![object, init];
            UIKeyboardAvoidingView { object }
        }
    }

    pub fn to_view(&self) -> UIView {
        unsafe { UIView::from_raw_retain(self.object) }
    }

    pub fn set_on_keyboard(&mut self, callback: UICallback<NSNumber>) {
        unsafe {
            let _: () = msg_send![self.object, setOnKeyboard: callback.as_raw()];
        }
    }
}

impl Raw for UIKeyboardAvoidingView {
    unsafe fn from_raw(object: *mut objc::runtime::Object) -> Self {
        UIKeyboardAvoidingView { object }
    }

    unsafe fn as_raw(&self) -> *mut objc::runtime::Object {
        self.object
    }
}

impl Clone for UIKeyboardAvoidingView {
    fn clone(&self) -> Self {
        unsafe { Self::from_raw_retain(self.as_raw()) }
    }
}

impl Drop for UIKeyboardAvoidingView {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
