use objc::runtime::*;
use objc::*;

use super::{PLYByEdge, PLYDimension, PLYLayout, PLYView};
use crate::coregraphics::CGRect;
use crate::uikit::UIScrollViewIndicatorStyle;
use crate::Raw;

/// A view that allows the scrolling and zooming of its contained views.
pub struct PLYScrollView {
    pub(super) object: *mut Object,
}

impl PLYScrollView {
    /// Initializes a newly allocated scroll view.
    pub fn new() -> PLYScrollView {
        unsafe {
            let mut object: *mut Object = msg_send![class!(PLYScrollView), alloc];
            object = msg_send![object, init];
            PLYScrollView { object }
        }
    }

    /// Upcasts this scroll view to a UIView.
    pub fn to_view(&self) -> PLYView {
        unsafe { PLYView::from_raw_retain(self.object) }
    }

    /// Sets the layout of the content view.
    pub fn set_content_layout(&mut self, callback: impl FnMut() -> CGRect + 'static) {
        let layout = PLYLayout::new(callback);

        unsafe {
            let _: () = msg_send![self.object, setContentLayout: layout];
        }
    }

    /// Sets the style of the scroll indicators.
    pub fn set_indicator_style(&mut self, style: UIScrollViewIndicatorStyle) {
        unsafe {
            let _: () = msg_send![self.object, setIndicatorStyle: style];
        }
    }

    /// Sets the custom distance that the content view is inset from the safe
    /// area or scroll view edges.
    pub fn set_scroll_padding(&mut self, padding: PLYByEdge<PLYDimension>) {
        unsafe {
            let _: () = msg_send![self.object, setScrollPadding: padding];
        }
    }

    /// Sets the distance the scroll indicators are inset from the edge of the
    /// scroll view.
    pub fn set_scrollbar_padding(&mut self, padding: PLYByEdge<PLYDimension>) {
        unsafe {
            let _: () = msg_send![self.object, setScrollbarPadding: padding];
        }
    }
}

impl Raw for PLYScrollView {
    unsafe fn from_raw(object: *mut Object) -> Self {
        PLYScrollView { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Clone for PLYScrollView {
    fn clone(&self) -> Self {
        unsafe { Self::from_raw_retain(self.as_raw()) }
    }
}

impl Drop for PLYScrollView {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
