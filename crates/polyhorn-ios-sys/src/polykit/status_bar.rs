use objc::runtime::*;
use objc::*;

use super::PLYWindow;
use crate::uikit::UIStatusBarStyle;
use crate::Raw;

/// Host overlay view that shows time, network, etc. in a bar at the top of the
/// screen.
pub struct PLYStatusBar {
    object: *mut Object,
}

impl PLYStatusBar {
    /// Returns a reference to the given window's status bar.
    pub fn new(window: &PLYWindow) -> PLYStatusBar {
        unsafe {
            let mut object: *mut Object = msg_send![class!(PLYStatusBar), alloc];
            object = msg_send![object, initWithWindow: window.as_raw()];
            PLYStatusBar { object }
        }
    }

    /// Sets the style of the status bar.
    pub fn set_style(&mut self, style: UIStatusBarStyle) {
        unsafe {
            let _: () = msg_send![self.object, setStyle: style];
        }
    }
}

impl Raw for PLYStatusBar {
    unsafe fn from_raw(object: *mut Object) -> Self {
        PLYStatusBar { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Drop for PLYStatusBar {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
