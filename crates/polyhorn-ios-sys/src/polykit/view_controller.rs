use objc::runtime::*;
use objc::*;

use super::{PLYCallback, PLYView};
use crate::foundation::NSNumber;
use crate::Raw;

/// An object that manages a view hierarchy of your UIKit app.
pub struct PLYViewController {
    object: *mut Object,
    view: PLYView,
}

impl PLYViewController {
    /// Initializes a newly allocated view controller.
    pub fn new() -> PLYViewController {
        unsafe {
            let mut object: *mut Object = msg_send![class!(PLYViewController), alloc];
            object = msg_send![object, init];
            PLYViewController::from_raw(object)
        }
    }

    /// The view that the controller manages.
    pub fn view_mut(&mut self) -> &mut PLYView {
        &mut self.view
    }

    /// Presents a view controller modally.
    pub fn present_view_controller(
        &mut self,
        view_controller: &PLYViewController,
        animated: bool,
        _completion: Option<()>,
    ) {
        unsafe {
            let _: () = msg_send![self.object, presentViewController: view_controller.as_raw()
                                                            animated: animated
                                                          completion: std::ptr::null_mut::<Object>()];
        }
    }

    /// Dismisses the view controller that was presented modally by the view
    /// controller.
    pub fn dismiss_view_controller(&mut self, animated: bool, _completion: Option<()>) {
        unsafe {
            let _: () = msg_send![self.object, dismissViewControllerAnimated: animated
                                                                  completion: std::ptr::null_mut::<Object>()];
        }
    }

    /// Sets a callback that will be invoked when this view controller
    /// disappears.
    pub fn set_on_did_disappear(&mut self, callback: &PLYCallback<NSNumber>) {
        unsafe {
            let _: () = msg_send![self.object, setOnDidDisappear: callback.as_raw()];
        }
    }
}

impl Raw for PLYViewController {
    unsafe fn from_raw(object: *mut Object) -> Self {
        PLYViewController {
            object,
            view: PLYView::from_raw_retain(msg_send![object, view]),
        }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Clone for PLYViewController {
    fn clone(&self) -> Self {
        unsafe { PLYViewController::from_raw_retain(self.as_raw()) }
    }
}

impl Drop for PLYViewController {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
