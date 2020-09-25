use objc::runtime::*;
use objc::*;

use super::PLYView;
use crate::foundation::NSAttributedString;
use crate::Raw;

/// An object that displays an editable text area in your interface.
pub struct PLYTextInputView {
    pub(super) object: *mut Object,
}

impl PLYTextInputView {
    /// Initializes a newly allocated text input view.
    pub fn new() -> PLYTextInputView {
        unsafe {
            let mut object: *mut Object = msg_send![class!(PLYTextInputView), alloc];
            object = msg_send![object, init];
            PLYTextInputView { object }
        }
    }

    /// Sets the styled string that displays when there is no other text in the
    /// text field.
    pub fn set_attributed_placeholder(&mut self, placeholder: &NSAttributedString) {
        unsafe {
            let _: () = msg_send![self.object, setAttributedPlaceholder: placeholder.as_raw()];
        }
    }

    /// Upcasts this text input view to a UIView.
    pub fn to_view(&self) -> PLYView {
        unsafe { PLYView::from_raw_retain(self.object) }
    }
}

impl Raw for PLYTextInputView {
    unsafe fn from_raw(object: *mut Object) -> Self {
        PLYTextInputView { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Clone for PLYTextInputView {
    fn clone(&self) -> Self {
        unsafe { Self::from_raw_retain(self.as_raw()) }
    }
}

impl Drop for PLYTextInputView {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
