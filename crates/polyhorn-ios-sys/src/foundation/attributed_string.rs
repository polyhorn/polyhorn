use objc::runtime::*;
use objc::*;

use super::{NSParagraphStyle, NSString};
use crate::coregraphics::{CGRect, CGSize};
use crate::uikit::{UIColor, UIFont};
use crate::Raw;

/// Attributes that you can apply to text in an attributed string.
pub struct NSAttributes {
    /// The font of the text. The value of this attribute is a `UIFont` object.
    /// Use this attribute to change the font for a range of text. If you do not
    /// specify this attribute, the string uses a 12-point Helvetica (Neue) font
    /// by default.
    pub font: UIFont,

    /// The color of the text. The value of this attribute is a `UIColor`
    /// object. Use this attribute to specify the color of the text during
    /// rendering. If you do not specify this attribute, the text is rendered in
    /// black.
    pub foreground_color: UIColor,

    /// The paragraph style of the text. The value of this attribute is an
    /// `NSParagraphStyle` object. Use this attribute to apply multiple
    /// attributes to a range of text. If you do not specify this attribute, the
    /// string uses the default paragraph attributes, as returned by the
    /// `defaultParagraphStyle` of `NSParagraphStyle`.
    pub paragraph_style: NSParagraphStyle,
}

extern "C" {
    /// The font of the text. The value of this attribute is a `UIFont` object.
    /// Use this attribute to change the font for a range of text. If you do not
    /// specify this attribute, the string uses a 12-point Helvetica (Neue) font
    /// by default.
    static NSFontAttributeName: *mut Object;

    /// The color of the text. The value of this attribute is a `UIColor`
    /// object. Use this attribute to specify the color of the text during
    /// rendering. If you do not specify this attribute, the text is rendered in
    /// black.
    static NSForegroundColorAttributeName: *mut Object;

    /// The paragraph style of the text. The value of this attribute is an
    /// `NSParagraphStyle` object. Use this attribute to apply multiple
    /// attributes to a range of text. If you do not specify this attribute, the
    /// string uses the default paragraph attributes, as returned by the
    /// `defaultParagraphStyle` of `NSParagraphStyle`.
    static NSParagraphStyleAttributeName: *mut Object;
}

impl NSAttributes {
    /// Converts a structure of attributes to a `NSDictionary`.
    pub fn into_dictionary(self) -> *mut Object {
        unsafe {
            let mut dictionary: *mut Object = msg_send![class!(NSMutableDictionary), alloc];
            dictionary = msg_send![dictionary, init];
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

/// A string with associated attributes (such as visual style, hyperlinks, or
/// accessibility data) for portions of its text.
pub struct NSAttributedString {
    object: *mut Object,
}

/// A mutable string object that also contains attributes (such as visual style,
/// hyperlinks, or accessibility data) associated with various portions of its
/// text content.
pub struct NSMutableAttributedString {
    object: *mut Object,
}

impl NSMutableAttributedString {
    /// Initializes a newly allocated mutable attributed string.
    pub fn new() -> NSMutableAttributedString {
        unsafe {
            let mut object: *mut Object = msg_send![class!(NSMutableAttributedString), alloc];
            object = msg_send![object, init];
            NSMutableAttributedString { object }
        }
    }

    /// Adds the characters and attributes of a given attributed string to the
    /// end of the receiver.
    pub fn append_attributed_string(&mut self, string: &NSAttributedString) {
        unsafe {
            let _: () = msg_send![self.object, appendAttributedString: string.as_raw()];
        }
    }
}

impl Raw for NSMutableAttributedString {
    unsafe fn from_raw(object: *mut Object) -> Self {
        NSMutableAttributedString { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
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
    /// Creates an attributed string with the characters of the specified string
    /// and no attribute information.
    pub fn new(value: &str) -> NSAttributedString {
        let string = NSString::from(value);

        unsafe {
            let mut object: *mut Object = msg_send![class!(NSAttributedString), alloc];
            object = msg_send![object, initWithString: string];
            NSAttributedString { object }
        }
    }

    /// Creates an attributed string with the specified string and attributes.
    pub fn with_attributes(value: &str, attributes: NSAttributes) -> NSAttributedString {
        let string = NSString::from(value);
        let attributes = attributes.into_dictionary();

        unsafe {
            let mut object: *mut Object = msg_send![class!(NSAttributedString), alloc];
            object = msg_send![object, initWithString: string.as_raw() attributes: attributes];
            objc_release(attributes);
            NSAttributedString { object }
        }
    }

    /// Returns the size necessary to draw the string.
    pub fn size(&self) -> CGSize {
        unsafe { msg_send![self.object, size] }
    }

    /// Returns the bounding rectangle necessary to draw the string.
    pub fn bounding_rect_with_size(&self, size: CGSize) -> CGRect {
        unsafe {
            msg_send![ self.object, boundingRectWithSize: size
                                                 options: 1 as usize
                                                 context: std::ptr::null_mut::<()>() ]
        }
    }
}

unsafe impl Send for NSAttributedString {}
unsafe impl Sync for NSAttributedString {}

impl Raw for NSAttributedString {
    unsafe fn from_raw(object: *mut Object) -> Self {
        NSAttributedString { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Clone for NSAttributedString {
    fn clone(&self) -> Self {
        unsafe { Self::from_raw_retain(self.as_raw()) }
    }
}

impl Drop for NSAttributedString {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
