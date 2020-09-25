use objc::runtime::*;
use objc::*;

use super::PLYCallback;
use crate::foundation::NSNumber;
use crate::{IntoRaw, Raw};

/// An object that provides keyframe animation capabilities for a view object.
pub struct PLYKeyframeAnimation {
    object: *mut Object,
}

impl PLYKeyframeAnimation {
    /// Initializes a newly allocated keyframe animation with the given
    /// duration, times and values.
    pub fn new<T>(duration: f64, times: &[f64], values: &[T]) -> PLYKeyframeAnimation
    where
        T: Copy + IntoRaw,
    {
        unsafe {
            let mut object: *mut Object = msg_send![class!(PLYKeyframeAnimation), alloc];
            let times = times.into_raw();
            let values = values.into_raw();
            object = msg_send![object, initWithDuration: duration
                                                  times: times.as_raw()
                                                 values: values.as_raw()];
            PLYKeyframeAnimation::from_raw(object)
        }
    }
}

impl Raw for PLYKeyframeAnimation {
    unsafe fn from_raw(object: *mut Object) -> Self {
        PLYKeyframeAnimation { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Drop for PLYKeyframeAnimation {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}

/// Handle that automatically stops a running animation once it is dropped.
pub struct PLYAnimationHandle {
    object: *mut Object,
}

impl PLYAnimationHandle {
    /// Sets a callback that will be invoked when the animation stops.
    pub fn set_on_stop(&mut self, callback: PLYCallback<NSNumber>) {
        unsafe {
            let _: () = msg_send![self.object, setOnStop: callback.as_raw()];
        }
    }
}

impl Raw for PLYAnimationHandle {
    unsafe fn from_raw(object: *mut Object) -> Self {
        PLYAnimationHandle { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Drop for PLYAnimationHandle {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
