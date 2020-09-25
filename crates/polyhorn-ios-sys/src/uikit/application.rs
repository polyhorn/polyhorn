use objc::runtime::*;
use objc::*;

use crate::Raw;

/// The centralized point of control and coordination of apps running in iOS.
pub struct UIApplication {
    object: *mut Object,
}

impl UIApplication {
    /// Returns the singleton app instance.
    pub fn shared() -> Self {
        unsafe {
            UIApplication {
                object: msg_send![class!(UIApplication), sharedApplication],
            }
        }
    }
}

impl Raw for UIApplication {
    unsafe fn from_raw(object: *mut Object) -> Self {
        UIApplication { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Clone for UIApplication {
    fn clone(&self) -> Self {
        unsafe { UIApplication::from_raw(self.as_raw()) }
    }
}

impl Drop for UIApplication {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
