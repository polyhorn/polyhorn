use objc::runtime::*;
use objc::*;

use crate::coregraphics::CGFloat;
use crate::Raw;

/// An object that stores color data and sometimes opacity.
pub struct UIColor {
    object: *mut Object,
}

impl UIColor {
    /// Creates a color object using the specified opacity and RGB component
    /// values.
    pub fn new(red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat) -> UIColor {
        unsafe {
            UIColor::from_raw_retain(msg_send![class!(UIColor), colorWithRed: red
                                                                       green: green
                                                                        blue: blue
                                                                       alpha: alpha])
        }
    }

    /// Returns the components that form the color in the RGB color space.
    pub fn get_components(&self) -> (CGFloat, CGFloat, CGFloat, CGFloat) {
        let mut red: CGFloat = 0.0;
        let mut green: CGFloat = 0.0;
        let mut blue: CGFloat = 0.0;
        let mut alpha: CGFloat = 0.0;

        unsafe {
            let _: () = msg_send![self.object, getRed: &mut red
                                                green: &mut green
                                                 blue: &mut blue
                                                alpha: &mut alpha];
        }

        (red, green, blue, alpha)
    }

    /// A color object with grayscale and alpha values that are both 0.0.
    pub fn clear() -> UIColor {
        unsafe { UIColor::from_raw_retain(msg_send![class!(UIColor), clearColor]) }
    }
}

/// UIColor is safe to send to other threads according to Apple's documentation.
unsafe impl Send for UIColor {}

/// UIColor is immutable and can be accessed from multiple threads at the same
/// time according to Apple's documentation.
unsafe impl Sync for UIColor {}

impl Raw for UIColor {
    unsafe fn from_raw(object: *mut Object) -> Self {
        UIColor { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Clone for UIColor {
    fn clone(&self) -> Self {
        unsafe { UIColor::from_raw_retain(self.as_raw()) }
    }
}

impl Drop for UIColor {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
