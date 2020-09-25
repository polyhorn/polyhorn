use objc::runtime::*;
use objc::*;

use super::PLYView;
use crate::foundation::NSAttributedString;
use crate::Raw;

/// A view that displays one or more lines of informational text.
pub struct PLYLabel {
    pub(super) object: *mut Object,
}

impl PLYLabel {
    /// Initializes a newly allocated label.
    pub fn new() -> PLYLabel {
        unsafe {
            let mut object: *mut Object = msg_send![class!(PLYLabel), alloc];
            object = msg_send![object, init];
            PLYLabel { object }
        }
    }

    /// Sets the styled that that the label displays.
    pub fn set_attributed_text(&mut self, string: &NSAttributedString) {
        unsafe {
            let _: () = msg_send![self.object, setAttributedText: string.as_raw()];
        }
    }

    /// Upcasts this label to a UIView.
    pub fn to_view(&self) -> PLYView {
        unsafe { PLYView::from_raw_retain(self.object) }
    }
}

impl Raw for PLYLabel {
    unsafe fn from_raw(object: *mut Object) -> Self {
        PLYLabel { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Clone for PLYLabel {
    fn clone(&self) -> Self {
        unsafe { Self::from_raw_retain(self.as_raw()) }
    }
}

impl Drop for PLYLabel {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
