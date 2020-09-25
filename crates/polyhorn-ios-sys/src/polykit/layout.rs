use objc::runtime::*;
use objc::*;

use crate::coregraphics::CGRect;
use crate::Raw;

/// A dynamic layout that can be queried from Objective-C.
pub struct PLYLayout {
    object: *mut Object,
}

pub struct PLYLayoutData(Box<dyn FnMut() -> CGRect>);

impl PLYLayout {
    /// Initializes a newly allocated dynamic layout with the given closure.
    pub fn new<F>(value: F) -> Self
    where
        F: FnMut() -> CGRect + 'static,
    {
        let data = PLYLayoutData(Box::new(value));

        unsafe {
            unsafe fn hook(data: *mut PLYLayoutData) -> CGRect {
                data.as_mut().unwrap().0()
            }

            unsafe fn free(data: *mut PLYLayoutData) {
                Box::from_raw(data);
            }

            let data = Box::into_raw(Box::new(data));

            let mut object: *mut Object = msg_send![class!(PLYLayout), alloc];

            object = msg_send![object, initWithHook: hook as usize
                                               free: free as usize
                                               data: data as usize];

            PLYLayout::from_raw(object)
        }
    }
}

impl Raw for PLYLayout {
    unsafe fn from_raw(object: *mut Object) -> Self {
        PLYLayout { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Drop for PLYLayout {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
