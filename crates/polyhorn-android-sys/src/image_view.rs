use jni::objects::JValue;

use super::{Bitmap, Context, Env, Object, Rect, Reference, View};

#[derive(Clone)]
pub struct ImageView {
    reference: Reference,
}

impl ImageView {
    pub fn new(env: &Env, context: impl Into<Context>) -> ImageView {
        unsafe {
            let context = context.into();

            let object = env.call_constructor(
                "com/glacyr/polyhorn/ImageView",
                "(Landroid/content/Context;)V",
                &[JValue::Object(context.as_reference().as_object()).into()],
            );

            ImageView {
                reference: env.retain(object),
            }
        }
    }

    pub fn set_image_bitmap(&mut self, env: &Env, bitmap: &Bitmap) {
        unsafe {
            env.call_method(
                self.reference.as_object(),
                "setImageBitmap",
                "(Landroid/graphics/Bitmap;)V",
                &[JValue::Object(bitmap.as_reference().as_object())],
            );
        }
    }

    pub fn set_frame(&mut self, env: &Env, frame: Rect) {
        unsafe {
            env.call_method(
                self.reference.as_object(),
                "setFrame",
                "(Lcom/glacyr/polyhorn/Rect;)V",
                &[JValue::Object(frame.as_reference().as_object())],
            );
        }
    }

    pub fn to_view(&self) -> View {
        View::from_reference(self.reference.clone())
    }
}
