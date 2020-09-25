use objc::runtime::*;
use objc::*;

use crate::{IntoRaw, Raw};

/// Declares the programmatic interface to objects that manage a modifiable
/// array of objects. This class adds insertion and deletion operations to the
/// basic array-handling behavior inherited from `NSArray`.
pub struct NSMutableArray {
    object: *mut Object,
}

impl NSMutableArray {
    /// Initializes a newly allocated array.
    pub fn new() -> NSMutableArray {
        unsafe {
            let mut object: *mut Object = msg_send![class!(NSMutableArray), alloc];
            object = msg_send![object, init];
            NSMutableArray::from_raw(object)
        }
    }

    /// Inserts a given object at the end of the array.
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
        unsafe { objc_release(self.object) }
    }
}

impl<T> IntoRaw for &[T]
where
    T: Copy + IntoRaw,
{
    type Raw = NSMutableArray;

    fn into_raw(self) -> Self::Raw {
        let mut result = NSMutableArray::new();

        for element in self {
            let element = element.into_raw();
            result.add_object(&element)
        }

        result
    }
}
