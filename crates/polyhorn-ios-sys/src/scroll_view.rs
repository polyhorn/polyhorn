use objc::runtime::*;
use objc::*;

use super::{CGRect, Raw, UIEdgeInsets, UILayout, UIView};

#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UIScrollViewIndicatorStyle {
    Default = 0,
    Black = 1,
    White = 2,
}

impl Default for UIScrollViewIndicatorStyle {
    fn default() -> Self {
        UIScrollViewIndicatorStyle::Default
    }
}

pub struct UIScrollView {
    pub(super) object: *mut Object,
}

impl UIScrollView {
    pub fn new() -> UIScrollView {
        unsafe {
            let object: *mut Object = msg_send![class!(PLYScrollView), alloc];
            let _: () = msg_send![object, init];
            UIScrollView { object }
        }
    }

    pub fn to_view(&self) -> UIView {
        unsafe { UIView::from_raw_retain(self.object) }
    }

    pub fn set_content_layout(&mut self, callback: impl FnMut() -> CGRect + 'static) {
        let layout = UILayout::new(callback);

        unsafe {
            let _: () = msg_send![self.object, setContentLayout: layout];
        }
    }

    pub fn set_indicator_style(&mut self, style: UIScrollViewIndicatorStyle) {
        unsafe {
            let _: () = msg_send![self.object, setIndicatorStyle: style];
        }
    }

    pub fn set_content_inset(&mut self, insets: UIEdgeInsets) {
        unsafe {
            let _: () = msg_send![self.object, setContentInset: insets];
        }
    }

    pub fn set_scroll_indicator_insets(&mut self, insets: UIEdgeInsets) {
        unsafe {
            let _: () = msg_send![self.object, setScrollIndicatorInsets: insets];
        }
    }
}

impl Raw for UIScrollView {
    unsafe fn from_raw(object: *mut objc::runtime::Object) -> Self {
        UIScrollView { object }
    }

    unsafe fn as_raw(&self) -> *mut objc::runtime::Object {
        self.object
    }
}

impl Clone for UIScrollView {
    fn clone(&self) -> Self {
        unsafe { Self::from_raw_retain(self.as_raw()) }
    }
}

impl Drop for UIScrollView {
    fn drop(&mut self) {
        unsafe {
            objc_release(self.object);
        }
    }
}
