use super::Raw;
use objc::runtime::*;
use objc::*;

pub struct UIApplication {
    object: *mut Object,
}

impl UIApplication {
    pub fn shared() -> Self {
        unsafe {
            UIApplication {
                object: msg_send![class!(UIApplication), sharedApplication],
            }
        }
    }
}

impl Raw for UIApplication {
    unsafe fn from_raw(object: *mut objc::runtime::Object) -> Self {
        UIApplication { object }
    }

    unsafe fn as_raw(&self) -> *mut objc::runtime::Object {
        self.object
    }
}

impl Clone for UIApplication {
    fn clone(&self) -> Self {
        unsafe { UIApplication::from_raw(msg_send![self.object, retain]) }
    }
}

impl Drop for UIApplication {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.object, release];
        }
    }
}
