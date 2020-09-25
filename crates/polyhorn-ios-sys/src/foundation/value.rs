use objc::runtime::*;
use objc::*;

use crate::quartzcore::CATransform3D;
use crate::{IntoRaw, Raw};

/// A simple container for a single C or Objective-C data item.
pub struct NSValue {
    object: *mut Object,
}

impl NSValue {
    /// Creates a new value object containing the specified CoreAnimation
    /// transform structure.
    pub fn with_transform_3d(transform: CATransform3D) -> NSValue {
        unsafe {
            let object: *mut Object = msg_send![class!(NSValue), valueWithCATransform3D: transform];
            NSValue::from_raw_retain(object)
        }
    }
}

impl Raw for NSValue {
    unsafe fn from_raw(object: *mut Object) -> Self {
        NSValue { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl IntoRaw for CATransform3D {
    type Raw = NSValue;

    fn into_raw(self) -> Self::Raw {
        NSValue::with_transform_3d(self)
    }
}
