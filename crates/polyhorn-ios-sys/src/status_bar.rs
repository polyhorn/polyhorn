use objc::runtime::*;
use objc::*;

use super::UIWindow;
use crate::Raw;

#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum UIStatusBarStyle {
    DarkContent = 0,
    LightContent = 1,
}

impl Default for UIStatusBarStyle {
    fn default() -> Self {
        UIStatusBarStyle::DarkContent
    }
}

pub struct UIStatusBar {
    object: *mut Object,
}

impl UIStatusBar {
    pub fn new(window: &UIWindow) -> UIStatusBar {
        unsafe {
            let mut object: *mut Object = msg_send![class!(PLYStatusBar), alloc];
            object = msg_send![object, initWithWindow: window.as_raw()];
            UIStatusBar { object }
        }
    }

    pub fn set_style(&mut self, style: UIStatusBarStyle) {
        unsafe {
            let _: () = msg_send![self.object, setStyle: style];
        }
    }
}

impl Raw for UIStatusBar {
    unsafe fn from_raw(object: *mut objc::runtime::Object) -> Self {
        UIStatusBar { object }
    }

    unsafe fn as_raw(&self) -> *mut objc::runtime::Object {
        self.object
    }
}

impl Drop for UIStatusBar {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
