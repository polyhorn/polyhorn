use crate::{IntoRaw, Raw};
use objc::runtime::{objc_release, Object};
use objc::*;

pub struct NSMutableArray {
    object: *mut Object,
}

impl NSMutableArray {
    pub fn new() -> NSMutableArray {
        unsafe {
            let mut object: *mut Object = msg_send![class!(NSMutableArray), alloc];
            object = msg_send![object, init];
            NSMutableArray::from_raw(object)
        }
    }

    pub fn add_object(&mut self, object: &impl Raw) {
        unsafe {
            let _: () = msg_send![self.object, addObject: object.as_raw()];
        }
    }
}

impl Raw for NSMutableArray {
    unsafe fn from_raw(object: *mut Object) -> Self {
        NSMutableArray { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Drop for NSMutableArray {
    fn drop(&mut self) {
        unsafe {
            objc_release(self.object);
        }
    }
}

impl<T> IntoRaw for [T]
where
    T: IntoRaw,
{
    type Raw = NSMutableArray;

    fn into_raw(&self) -> Self::Raw {
        let mut result = NSMutableArray::new();

        for element in self {
            let element = element.into_raw();
            result.add_object(&element)
        }

        result
    }
}
