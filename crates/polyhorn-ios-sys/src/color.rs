use super::Raw;
use fsize::fsize;
use objc::runtime::*;
use objc::*;

pub struct UIColor {
    object: *mut Object,
}

macro_rules! color {
    ($rs:ident, $objc:ident) => {
        pub fn $rs() -> UIColor {
            unsafe {
                UIColor {
                    object: msg_send![class!(UIColor), $objc],
                }
            }
        }
    };
}

impl UIColor {
    pub fn new(red: fsize, green: fsize, blue: fsize, alpha: fsize) -> UIColor {
        unsafe {
            UIColor::from_raw(msg_send![class!(UIColor), colorWithRed: red
                                                                        green: green
                                                                         blue: blue
                                                                        alpha: alpha])
        }
    }

    pub fn get_components(&self) -> (fsize, fsize, fsize, fsize) {
        let mut red: fsize = 0.0;
        let mut green: fsize = 0.0;
        let mut blue: fsize = 0.0;
        let mut alpha: fsize = 0.0;

        unsafe {
            let _: () = msg_send![self.object, getRed: &mut red
                                                green: &mut green
                                                 blue: &mut blue
                                                alpha: &mut alpha];
        }

        (red, green, blue, alpha)
    }

    color!(clear, clearColor);

    color!(black, blackColor);
    color!(blue, blueColor);
    color!(brown, brownColor);
    color!(cyan, cyanColor);
    color!(dark_gray, darkGrayColor);
    color!(gray, grayColor);
    color!(green, greenColor);
    color!(red, redColor);
}

/// UIColor is safe to send to other threads according to Apple's documentation.
unsafe impl Send for UIColor {}

/// UIColor is immutable and can be accessed from multiple threads at the same
/// time according to Apple's documentation.
unsafe impl Sync for UIColor {}

impl Raw for UIColor {
    unsafe fn from_raw(object: *mut objc::runtime::Object) -> Self {
        UIColor { object }
    }

    unsafe fn as_raw(&self) -> *mut objc::runtime::Object {
        self.object
    }
}

impl Clone for UIColor {
    fn clone(&self) -> Self {
        unsafe {
            UIColor {
                object: msg_send![self.object, retain],
            }
        }
    }
}

impl Drop for UIColor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.object, release];
        }
    }
}
