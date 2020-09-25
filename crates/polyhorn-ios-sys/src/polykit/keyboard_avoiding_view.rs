use objc::runtime::*;
use objc::*;

use super::{PLYCallback, PLYView};
use crate::foundation::NSNumber;
use crate::Raw;

/// A view that invoke a callback when the keyboard appears, changes its
/// appearance or disappears.
pub struct PLYKeyboardAvoidingView {
    pub(super) object: *mut Object,
}

impl PLYKeyboardAvoidingView {
    /// Initializes a newly allocated keyboard avoiding view.
    pub fn new() -> PLYKeyboardAvoidingView {
        unsafe {
            let mut object: *mut Object = msg_send![class!(PLYKeyboardAvoidingView), alloc];
            object = msg_send![object, init];
            PLYKeyboardAvoidingView { object }
        }
    }

    /// Upcasts this keyboard avoiding view to a UIView.
    pub fn to_view(&self) -> PLYView {
        unsafe { PLYView::from_raw_retain(self.object) }
    }

    /// Sets a callback that is invoked whenever the keyboard appears, changes
    /// its appearance or disappears.
    pub fn set_on_keyboard(&mut self, callback: PLYCallback<NSNumber>) {
        unsafe {
            let _: () = msg_send![self.object, setOnKeyboard: callback.as_raw()];
        }
    }
}

impl Raw for PLYKeyboardAvoidingView {
    unsafe fn from_raw(object: *mut Object) -> Self {
        PLYKeyboardAvoidingView { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Clone for PLYKeyboardAvoidingView {
    fn clone(&self) -> Self {
        unsafe { Self::from_raw_retain(self.as_raw()) }
    }
}

impl Drop for PLYKeyboardAvoidingView {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
