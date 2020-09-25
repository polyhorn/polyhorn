use objc::runtime::*;
use objc::*;

use super::{
    PLYAnimationHandle, PLYCallback, PLYCornerRadii, PLYEdgeInsets, PLYKeyframeAnimation,
    PLYLayout, PLYWindow,
};
use crate::coregraphics::{CGFloat, CGRect};
use crate::foundation::NSString;
use crate::quartzcore::CATransform3D;
use crate::uikit::UIColor;
use crate::Raw;

/// An object that manages the content for a rectangular area on the screen.
pub struct PLYView {
    pub(super) object: *mut Object,
}

impl PLYView {
    /// Initializes a newly allocated view.
    pub fn new() -> PLYView {
        unsafe {
            let mut object: *mut Object = msg_send![class!(PLYView), alloc];
            object = msg_send![object, init];
            PLYView { object }
        }
    }

    /// The insets that you use to determine the safe area for this view.
    pub fn safe_area_insets(&self) -> PLYEdgeInsets {
        unsafe { msg_send![self.object, safeAreaInsets] }
    }

    /// Sets the view's alpha value.
    pub fn set_alpha(&mut self, alpha: CGFloat) {
        unsafe {
            let _: () = msg_send![self.object, setAlpha: alpha];
        }
    }

    /// Sets the view's background color.
    pub fn set_background_color(&mut self, color: UIColor) {
        unsafe {
            let _: () = msg_send![self.object, setOpaqueBackgroundColor: color.as_raw() ];
        }
    }

    /// The frame rectangle, which describes the view's location and size in its
    /// superview's coordinate system.
    pub fn frame(&self) -> CGRect {
        unsafe { msg_send![self.object, frame] }
    }

    /// Sets the frame rectangle, which describes the view's location and size
    /// in its superview's coordinate system.
    pub fn set_frame(&mut self, frame: CGRect) {
        unsafe {
            let _: () = msg_send![self.object, setFrame: frame];
        }
    }

    /// The radius to use when drawing rounded corners for the layer's
    /// background.
    pub fn set_corner_radii(&mut self, corner_radii: PLYCornerRadii) {
        unsafe {
            let _: () = msg_send![self.object, setCornerRadii: corner_radii];
        }
    }

    /// The receiver's window object, or `None` if it has none.
    pub fn window(&self) -> Option<PLYWindow> {
        unsafe {
            let object: *mut Object = msg_send![self.object, window];

            match object.is_null() {
                false => Some(PLYWindow::from_raw_retain(object)),
                true => None,
            }
        }
    }

    /// Adds a view to the end of the receiver's list of subviews.
    pub fn add_subview(&mut self, subview: &PLYView) {
        unsafe {
            let _: () = msg_send![self.object, addSubview: subview.as_raw()];
        }
    }

    /// Unlinks the view from its superview and its window, and removes it from
    /// the responder chain.
    pub fn remove_from_superview(&mut self) {
        unsafe {
            let _: () = msg_send![self.object, removeFromSuperview];
        }
    }

    /// Sets the view's layout.
    pub fn set_layout(&mut self, callback: impl FnMut() -> CGRect + 'static) {
        let layout = PLYLayout::new(callback);

        unsafe {
            let _: () = msg_send![self.object, setLayout: layout.as_raw()];
        }
    }

    /// A Boolean value that determines whether the view is hidden.
    pub fn set_hidden(&mut self, hidden: bool) {
        unsafe {
            let _: () = msg_send![self.object, setHidden: hidden];
        }
    }

    /// Specifies the transform applied to the view, relative to the center of
    /// its bounds.
    pub fn set_transform(&mut self, transform: CATransform3D) {
        unsafe {
            let object: *mut Object = msg_send![self.object, layer];
            let _: () = msg_send![object, setTransform: transform];
        }
    }

    /// Invalidates the current layout of the receiver and triggers a layout
    /// update during the next update cycle.
    pub fn set_needs_layout(&mut self) {
        unsafe {
            let _: () = msg_send![self.object, setNeedsLayout];
        }
    }

    /// Sets a callback that is invoked when the user's interaction with this
    /// view is cancelled.
    pub fn set_on_pointer_cancel(&mut self, callback: PLYCallback<NSString>) {
        unsafe {
            let _: () = msg_send![self.object, setOnPointerCancel: callback.as_raw()];
        }
    }

    /// Sets a callback that is invoked when the user starts interacting with
    /// this view.
    pub fn set_on_pointer_down(&mut self, callback: PLYCallback<NSString>) {
        unsafe {
            let _: () = msg_send![self.object, setOnPointerDown: callback.as_raw()];
        }
    }

    /// Sets a callback that is invoked when the user ends interacting with this
    /// view.
    pub fn set_on_pointer_up(&mut self, callback: PLYCallback<NSString>) {
        unsafe {
            let _: () = msg_send![self.object, setOnPointerUp: callback.as_raw()];
        }
    }

    /// Adds the specified animation object to the view's render tree.
    pub fn add_animation(
        &mut self,
        animation: PLYKeyframeAnimation,
        key_path: &str,
    ) -> PLYAnimationHandle {
        let key_path = NSString::from(key_path);
        unsafe {
            PLYAnimationHandle::from_raw_retain(
                msg_send![self.object, addKeyframeAnimation: animation.as_raw()
                                                 forKeyPath: key_path.as_raw()],
            )
        }
    }
}

impl Raw for PLYView {
    unsafe fn from_raw(object: *mut Object) -> Self {
        PLYView { object }
    }

    unsafe fn as_raw(&self) -> *mut Object {
        self.object
    }
}

impl Clone for PLYView {
    fn clone(&self) -> Self {
        unsafe { Self::from_raw_retain(self.as_raw()) }
    }
}

impl Drop for PLYView {
    fn drop(&mut self) {
        unsafe { objc_release(self.object) }
    }
}
