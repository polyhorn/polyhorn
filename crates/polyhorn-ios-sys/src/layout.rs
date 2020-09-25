use super::{CGRect, Raw};
use objc::runtime::*;
use objc::*;

pub struct UILayout {
    object: *mut Object,
}

pub struct UILayoutData(Box<dyn FnMut() -> CGRect>);

impl UILayout {
    pub fn new<F>(value: F) -> Self
    where
        F: FnMut() -> CGRect + 'static,
    {
        let data = UILayoutData(Box::new(value));

        unsafe {
            unsafe fn hook(data: *mut UILayoutData) -> CGRect {
                data.as_mut().unwrap().0()
            }

            unsafe fn free(data: *mut UILayoutData) {
                Box::from_raw(data);
            }

            let data = Box::into_raw(Box::new(data));

            let mut object: *mut Object = msg_send![class!(PLYLayout), alloc];

            object = msg_send![object, initWithHook: hook as usize
                                               free: free as usize
                                               data: data as usize];

            UILayout::from_raw(object)
        }
    }
}

impl Raw for UILayout {
    unsafe fn from_raw(object: *mut objc::runtime::Object) -> Self {
        UILayout { object }
    }

    unsafe fn as_raw(&self) -> *mut objc::runtime::Object {
        self.object
    }
}

impl Drop for UILayout {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.object, release];
        }
    }
}
