use objc::runtime::*;

/// Represents a type that can be converted to a Rust wrapper of a raw
/// Objective-C type.
pub trait IntoRaw {
    /// The Rust wrapper of a raw Objective-C type that this type can be
    /// converted into.
    type Raw: Raw;

    /// Converts this type into a Rust wrapper of an Objective-C type.
    fn into_raw(self) -> Self::Raw;
}

/// A trait that is implemented by Rust wrappers of Objective-C types.
pub trait Raw {
    /// Returns a new instance of this Rust type from the given raw Objective-C
    /// object without affecting its retain count.
    unsafe fn from_raw(object: *mut Object) -> Self;

    /// Returns the underlying pointer to the raw Objective-C object.
    unsafe fn as_raw(&self) -> *mut Object;

    /// Returns a new instance of this Rust type that retains the given raw
    /// Objective-C object.
    unsafe fn from_raw_retain(object: *mut Object) -> Self
    where
        Self: Sized,
    {
        Self::from_raw(objc_retain(object))
    }
}
