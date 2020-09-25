use objc::runtime::{objc_release, Object};
use objc::*;

use crate::{IntoRaw, Raw};

/// An object wrapper for primitive scalar numeric values.
pub struct NSNumber {
    object: *mut Object,
}

impl NSNumber {
    /// The number object's value expressed as a `float`, converted as
    /// necessary.
    pub fn float_value(&self) -> f32 {
        unsafe { msg_send![self.object, floatValue] }
    }
}

impl From<f32> for NSNumber {
    fn from(value: f32) -> Self {
        unsafe {
            let mut object: *mut Object = msg_send![class!(NSNumber), alloc];
            object = msg_send![object, initWithFloat: value];
            NSNumber::from_raw(object)
        }
    }
}

impl From<f64> for NSNumber {
    fn from(value: f64) -> Self {
        unsafe {
            let mut object: *mut Object = msg_send![class!(NSNumber), alloc];
            object = msg_send![object, initWithDouble: value];
            NSNumber::from_raw(object)
        }
    }
}

impl IntoRaw for f32 {
    type Raw = NSNumber;

    fn into_raw(self) -> Self::Raw {
        self.into()
    }
}

impl IntoRaw for f64 {
    type Raw = NSNumber;

    fn into_raw(self) -> Self::Raw {
        self.into()
    }
}

impl Raw for NSNumber {
    unsafe fn from_raw(object: *mut Object) -> Self {
        NSNumber { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Drop for NSNumber {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
