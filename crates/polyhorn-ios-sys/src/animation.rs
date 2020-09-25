use super::{IntoRaw, Raw, UICallback};
use crate::foundation::NSNumber;
use objc::runtime::*;
use objc::*;

pub struct UIKeyframeAnimation {
    object: *mut Object,
}

impl UIKeyframeAnimation {
    pub fn new<T>(duration: f64, times: &[f64], values: &[T]) -> UIKeyframeAnimation
    where
        T: IntoRaw,
    {
        unsafe {
            let mut object: *mut Object = msg_send![class!(PLYKeyframeAnimation), alloc];
            let times = times.into_raw();
            let values = values.into_raw();
            object = msg_send![object, initWithDuration: duration
                                                  times: times.as_raw()
                                                 values: values.as_raw()];
            UIKeyframeAnimation::from_raw(object)
        }
    }
}

impl Raw for UIKeyframeAnimation {
    unsafe fn from_raw(object: *mut objc::runtime::Object) -> Self {
        UIKeyframeAnimation { object }
    }

    unsafe fn as_raw(&self) -> *mut objc::runtime::Object {
        self.object
    }
}

impl Drop for UIKeyframeAnimation {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}

pub struct UIAnimationHandle {
    object: *mut Object,
}

impl UIAnimationHandle {
    pub fn set_on_stop(&mut self, callback: UICallback<NSNumber>) {
        unsafe {
            let _: () = msg_send![self.object, setOnStop: callback.as_raw()];
        }
    }
}

impl Raw for UIAnimationHandle {
    unsafe fn from_raw(object: *mut objc::runtime::Object) -> Self {
        UIAnimationHandle { object }
    }

    unsafe fn as_raw(&self) -> *mut objc::runtime::Object {
        self.object
    }
}

impl Drop for UIAnimationHandle {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
