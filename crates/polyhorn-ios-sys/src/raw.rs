use objc::*;

pub trait IntoRaw {
    type Raw: Raw;

    fn into_raw(&self) -> Self::Raw;
}

pub trait Raw {
    unsafe fn from_raw(object: *mut objc::runtime::Object) -> Self;
    unsafe fn as_raw(&self) -> *mut objc::runtime::Object;

    unsafe fn from_raw_retain(object: *mut objc::runtime::Object) -> Self
    where
        Self: Sized,
    {
        Self::from_raw(msg_send![object, retain])
    }
}
