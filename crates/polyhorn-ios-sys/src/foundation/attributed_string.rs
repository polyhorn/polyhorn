use objc::runtime::*;
use objc::*;

use super::{NSParagraphStyle, NSString};
use crate::{CGRect, CGSize, Raw, UIColor, UIFont};

pub struct NSAttributes {
    pub font: UIFont,
    pub foreground_color: UIColor,
    pub paragraph_style: NSParagraphStyle,
}

extern "C" {
    static NSFontAttributeName: *mut Object;
    static NSForegroundColorAttributeName: *mut Object;
    static NSParagraphStyleAttributeName: *mut Object;
}

impl NSAttributes {
    pub fn into_dictionary(self) -> *mut Object {
        unsafe {
            let dictionary: *mut Object = msg_send![class!(NSMutableDictionary), dictionary];
            let _: () = msg_send![dictionary, setObject: self.font.as_raw()
                                                 forKey: NSFontAttributeName];
            let _: () = msg_send![dictionary, setObject: self.foreground_color.as_raw()
                                                 forKey: NSForegroundColorAttributeName];
            let _: () = msg_send![dictionary, setObject: self.paragraph_style.as_raw()
                                                 forKey: NSParagraphStyleAttributeName];
            dictionary
        }
    }
}

pub struct NSAttributedString {
    object: *mut Object,
}

pub struct NSMutableAttributedString {
    object: *mut Object,
}

impl NSMutableAttributedString {
    pub fn new() -> NSMutableAttributedString {
        unsafe {
            let object: *mut Object = msg_send![class!(NSMutableAttributedString), alloc];
            let _: () = msg_send![object, init];
            NSMutableAttributedString { object }
        }
    }

    pub fn append_attributed_string(&mut self, string: &NSAttributedString) {
        unsafe {
            let _: () = msg_send![self.object, appendAttributedString: string.as_raw()];
        }
    }
}

impl Raw for NSMutableAttributedString {
    unsafe fn from_raw(object: *mut objc::runtime::Object) -> Self {
        NSMutableAttributedString { object }
    }

    unsafe fn as_raw(&self) -> *mut objc::runtime::Object {
        self.object
    }
}

impl Drop for NSMutableAttributedString {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}

impl Into<NSAttributedString> for NSMutableAttributedString {
    fn into(self) -> NSAttributedString {
        unsafe { NSAttributedString::from_raw_retain(self.as_raw()) }
    }
}

impl NSAttributedString {
    pub fn new(value: &str) -> NSAttributedString {
        let string = NSString::from(value);

        unsafe {
            let mut object: *mut Object = msg_send![class!(NSAttributedString), alloc];
            object = msg_send![object, initWithString: string];
            NSAttributedString { object }
        }
    }

    pub fn with_attributes(value: &str, attributes: NSAttributes) -> NSAttributedString {
        let string = NSString::from(value);
        let attributes = attributes.into_dictionary();

        unsafe {
            let mut object: *mut Object = msg_send![class!(NSAttributedString), alloc];
            object = msg_send![object, initWithString: string.as_raw() attributes: attributes];
            NSAttributedString { object }
        }
    }

    pub fn size(&self) -> CGSize {
        unsafe { msg_send![self.object, size] }
    }

    pub fn bounding_rect_with_size(&self, size: CGSize) -> CGRect {
        unsafe {
            msg_send![ self.object, boundingRectWithSize: size
                                                 options: 1 as usize
                                                 context: std::ptr::null_mut::<()>() ]
        }
    }
}

impl Raw for NSAttributedString {
    unsafe fn from_raw(object: *mut objc::runtime::Object) -> Self {
        NSAttributedString { object }
    }

    unsafe fn as_raw(&self) -> *mut objc::runtime::Object {
        self.object
    }
}

impl Clone for NSAttributedString {
    fn clone(&self) -> Self {
        unsafe { Self::from_raw(msg_send![self.as_raw(), retain]) }
    }
}

impl Drop for NSAttributedString {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.object, release];
        }
    }
}
