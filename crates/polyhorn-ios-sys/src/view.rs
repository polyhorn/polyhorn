use fsize::fsize;
use objc::runtime::*;
use objc::*;

use super::foundation::NSString;
use super::{
    CGRect, Raw, UIAnimationHandle, UICallback, UIColor, UIKeyframeAnimation, UILayout, UIWindow,
};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CATransform3D {
    pub m11: fsize,
    pub m12: fsize,
    pub m13: fsize,
    pub m14: fsize,
    pub m21: fsize,
    pub m22: fsize,
    pub m23: fsize,
    pub m24: fsize,
    pub m31: fsize,
    pub m32: fsize,
    pub m33: fsize,
    pub m34: fsize,
    pub m41: fsize,
    pub m42: fsize,
    pub m43: fsize,
    pub m44: fsize,
}

impl Default for CATransform3D {
    fn default() -> Self {
        CATransform3D {
            m11: 1.0,
            m12: 0.0,
            m13: 0.0,
            m14: 0.0,
            m21: 0.0,
            m22: 1.0,
            m23: 0.0,
            m24: 0.0,
            m31: 0.0,
            m32: 0.0,
            m33: 1.0,
            m34: 0.0,
            m41: 0.0,
            m42: 0.0,
            m43: 0.0,
            m44: 1.0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UIDimension {
    pub kind: UIDimensionKind,
    pub value: fsize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum UIDimensionKind {
    Pixels,
    Percentage,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UIPoint {
    pub x: UIDimension,
    pub y: UIDimension,
}

impl UIPoint {
    pub fn new(x: UIDimension, y: UIDimension) -> UIPoint {
        UIPoint { x, y }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UICornerRadii {
    pub top_leading: UIPoint,
    pub top_trailing: UIPoint,
    pub bottom_trailing: UIPoint,
    pub bottom_leading: UIPoint,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct UIEdgeInsets {
    pub top: fsize,
    pub left: fsize,
    pub bottom: fsize,
    pub right: fsize,
}

pub struct UIView {
    pub(super) object: *mut Object,
}

impl UIView {
    pub fn new() -> UIView {
        unsafe {
            let object: *mut Object = msg_send![class!(PLYView), alloc];
            let _: () = msg_send![object, init];
            UIView { object }
        }
    }

    pub fn safe_area_insets(&self) -> UIEdgeInsets {
        unsafe { msg_send![self.object, safeAreaInsets] }
    }

    pub fn set_alpha(&mut self, alpha: fsize) {
        unsafe {
            let _: () = msg_send![self.object, setAlpha: alpha];
        }
    }

    pub fn set_background_color(&mut self, color: UIColor) {
        unsafe {
            let _: () = msg_send![self.object, setOpaqueBackgroundColor: color.as_raw() ];
        }
    }

    pub fn frame(&self) -> CGRect {
        unsafe { msg_send![self.object, frame] }
    }

    pub fn set_frame(&mut self, frame: CGRect) {
        unsafe {
            let _: () = msg_send![self.object, setFrame: frame];
        }
    }

    pub fn set_corner_radii(&mut self, corner_radii: UICornerRadii) {
        unsafe {
            let _: () = msg_send![self.object, setCornerRadii: corner_radii];
        }
    }

    pub fn window(&self) -> Option<UIWindow> {
        unsafe {
            let object: *mut Object = msg_send![self.object, window];

            match object.is_null() {
                false => Some(UIWindow::from_raw_retain(object)),
                true => None,
            }
        }
    }

    pub fn add_subview(&mut self, subview: &UIView) {
        unsafe {
            let _: () = msg_send![self.object, addSubview: subview.as_raw()];
        }
    }

    pub fn remove_from_superview(&mut self) {
        unsafe {
            let _: () = msg_send![self.object, removeFromSuperview];
        }
    }

    pub fn set_layout(&mut self, callback: impl FnMut() -> CGRect + 'static) {
        let layout = UILayout::new(callback);

        unsafe {
            let _: () = msg_send![self.object, setLayout: layout.as_raw()];
        }
    }

    pub fn set_hidden(&mut self, hidden: bool) {
        unsafe {
            let _: () = msg_send![self.object, setHidden: hidden];
        }
    }

    pub fn set_transform_translation_x(&mut self, x: f32) {
        unsafe {
            let transform = CATransform3D {
                m41: x as _,
                ..Default::default()
            };
            let object: *mut Object = msg_send![self.object, layer];
            let _: () = msg_send![object, setTransform: transform];
        }
    }

    pub fn set_needs_layout(&mut self) {
        unsafe {
            let _: () = msg_send![self.object, setNeedsLayout];
        }
    }

    pub fn set_on_pointer_cancel(&mut self, callback: UICallback<NSString>) {
        unsafe {
            let _: () = msg_send![self.object, setOnPointerCancel: callback.as_raw()];
        }
    }

    pub fn set_on_pointer_down(&mut self, callback: UICallback<NSString>) {
        unsafe {
            let _: () = msg_send![self.object, setOnPointerDown: callback.as_raw()];
        }
    }

    pub fn set_on_pointer_up(&mut self, callback: UICallback<NSString>) {
        unsafe {
            let _: () = msg_send![self.object, setOnPointerUp: callback.as_raw()];
        }
    }

    pub fn add_animation(
        &mut self,
        animation: UIKeyframeAnimation,
        key_path: &str,
    ) -> UIAnimationHandle {
        let key_path = NSString::from(key_path);
        unsafe {
            UIAnimationHandle::from_raw_retain(
                msg_send![self.object, addKeyframeAnimation: animation.as_raw()
                                                 forKeyPath: key_path.as_raw()],
            )
        }
    }
}

impl Raw for UIView {
    unsafe fn from_raw(object: *mut objc::runtime::Object) -> Self {
        UIView { object }
    }

    unsafe fn as_raw(&self) -> *mut objc::runtime::Object {
        self.object
    }
}

impl Clone for UIView {
    fn clone(&self) -> Self {
        unsafe { Self::from_raw(msg_send![self.as_raw(), retain]) }
    }
}

impl Drop for UIView {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.object, release];
        }
    }
}
