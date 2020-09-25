use objc::runtime::*;
use objc::*;

use super::{Raw, UICallback, UIView};
use crate::foundation::NSNumber;

pub struct UIViewController {
    object: *mut Object,
    view: UIView,
}

impl UIViewController {
    pub fn new() -> UIViewController {
        unsafe {
            let object: *mut Object = msg_send![class!(PLYViewController), alloc];
            let _: () = msg_send![object, init];
            UIViewController::from_raw(object)
        }
    }

    pub fn view_mut(&mut self) -> &mut UIView {
        &mut self.view
    }

    pub fn present_view_controller(
        &mut self,
        view_controller: &UIViewController,
        animated: bool,
        _completion: Option<()>,
    ) {
        unsafe {
            let _: () = msg_send![self.object, presentViewController: view_controller.as_raw()
                                                            animated: animated
                                                          completion: std::ptr::null_mut::<Object>()];
        }
    }

    pub fn dismiss_view_controller(&mut self, animated: bool, _completion: Option<()>) {
        unsafe {
            let _: () = msg_send![self.object, dismissViewControllerAnimated: animated
                                                                  completion: std::ptr::null_mut::<Object>()];
        }
    }

    pub fn set_on_did_disappear(&mut self, callback: &UICallback<NSNumber>) {
        unsafe {
            let _: () = msg_send![self.object, setOnDidDisappear: callback.as_raw()];
        }
    }
}

impl Raw for UIViewController {
    unsafe fn from_raw(object: *mut objc::runtime::Object) -> Self {
        UIViewController {
            object,
            view: UIView::from_raw_retain(msg_send![object, view]),
        }
    }

    unsafe fn as_raw(&self) -> *mut objc::runtime::Object {
        self.object
    }
}

impl Clone for UIViewController {
    fn clone(&self) -> Self {
        unsafe { UIViewController::from_raw_retain(self.as_raw()) }
    }
}

impl Drop for UIViewController {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
