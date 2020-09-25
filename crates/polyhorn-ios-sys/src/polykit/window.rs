use objc::runtime::*;
use objc::*;

use super::PLYViewController;
use crate::Raw;

/// The backdrop of your app's user interface and the object that dispatches
/// events to your views.
pub struct PLYWindow {
    object: *mut Object,
}

impl PLYWindow {
    /// Initializes a newly allocated window.
    pub fn new() -> PLYWindow {
        unsafe {
            let mut object: *mut Object = msg_send![class!(UIWindow), alloc];
            object = msg_send![object, init];

            PLYWindow { object }
        }
    }

    /// The app's key window.
    pub fn key_window() -> PLYWindow {
        unsafe {
            let object: *mut Object = msg_send![class!(UIApplication), sharedApplication];
            let object: *mut Object = msg_send![object, keyWindow];

            PLYWindow::from_raw_retain(object)
        }
    }

    /// Shows the window and makes it the key window.
    pub fn make_key_and_visible(&mut self) {
        unsafe {
            let _: () = msg_send![self.object, makeKeyAndVisible];
        }
    }

    /// The root view controller for the window.
    pub fn root_view_controller(&self) -> PLYViewController {
        unsafe { PLYViewController::from_raw_retain(msg_send![self.object, rootViewController]) }
    }

    /// Sets the root view controller for the window.
    pub fn set_root_view_controller(&mut self, view_controller: PLYViewController) {
        unsafe {
            let _: () = msg_send![self.object, setRootViewController: view_controller.as_raw()];
        }
    }
}

impl Clone for PLYWindow {
    fn clone(&self) -> Self {
        unsafe { PLYWindow::from_raw_retain(self.as_raw()) }
    }
}

impl Raw for PLYWindow {
    unsafe fn from_raw(object: *mut Object) -> Self {
        Self { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Drop for PLYWindow {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
