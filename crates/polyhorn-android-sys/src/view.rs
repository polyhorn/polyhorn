use jni::objects::JValue;

use super::{Color, Context, Env, Object, Rect, Reference};

#[derive(Clone)]
pub struct View {
    reference: Reference,
}

impl View {
    pub fn new(env: &Env, context: impl Into<Context>) -> View {
        unsafe {
            let context = context.into();

            let object = env.call_constructor(
                "com/glacyr/polyhorn/View",
                "(Landroid/content/Context;)V",
                &[JValue::Object(context.as_reference().as_object()).into()],
            );

            View {
                reference: env.retain(object),
            }
        }
    }

    pub fn set_background_color(&mut self, env: &Env, color: Color) {
        unsafe {
            match color {
                Color::Unmanaged(value) => env.call_method(
                    self.reference.as_object(),
                    "setBackgroundColor",
                    "(I)V",
                    &[JValue::Int(value)],
                ),
                Color::Managed(_) => todo!("Managed colors are not yet available on Android."),
            };
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

    pub fn bounds(&self, env: &Env) -> Rect {
        unsafe {
            match env.call_method(
                self.reference.as_object(),
                "getBounds",
                "()Lcom/glacyr/polyhorn/Rect;",
                &[],
            ) {
                JValue::Object(value) => Rect::from_reference(env.retain(value)),
                _ => unreachable!(),
            }
        }
    }

    pub fn add_view(&mut self, env: &Env, view: &View) {
        unsafe {
            env.call_method(
                self.reference.as_object(),
                "addView",
                "(Landroid/view/View;)V",
                &[JValue::Object(view.as_reference().as_object()).into()],
            );
        }
    }
}

impl Object for View {
    fn from_reference(reference: Reference) -> Self {
        View { reference }
    }

    fn as_reference(&self) -> &Reference {
        &self.reference
    }
}
