use super::{Raw, UIViewController};
use objc::runtime::*;
use objc::*;

pub struct UIWindow {
    object: *mut Object,
}

impl UIWindow {
    pub fn new() -> UIWindow {
        unsafe {
            let mut object: *mut Object = msg_send![class!(UIWindow), alloc];
            object = msg_send![object, init];

            UIWindow { object }
        }
    }

    pub fn key() -> UIWindow {
        unsafe {
            let object: *mut Object = msg_send![class!(UIWindow), keyWindow];

            UIWindow::from_raw_retain(object)
        }
    }

    pub fn make_key_and_visible(&mut self) {
        unsafe {
            let _: () = msg_send![self.object, makeKeyAndVisible];
        }
    }

    pub unsafe fn raw(&self) -> *mut Object {
        self.object
    }

    pub fn root_view_controller(&self) -> UIViewController {
        unsafe { UIViewController::from_raw_retain(msg_send![self.object, rootViewController]) }
    }

    pub fn set_root_view_controller(&mut self, view_controller: UIViewController) {
        unsafe {
            let _: () = msg_send![self.object, setRootViewController: view_controller.as_raw()];
        }
    }
}

impl Raw for UIWindow {
    unsafe fn from_raw(object: *mut objc::runtime::Object) -> Self {
        Self { object }
    }

    unsafe fn as_raw(&self) -> *mut objc::runtime::Object {
        self.object
    }
}

impl Clone for UIWindow {
    fn clone(&self) -> Self {
        unsafe { Self::from_raw_retain(self.as_raw()) }
    }
}

impl Drop for UIWindow {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.object, release];
        }
    }
}
