use jni::objects::JValue;

use super::{Env, Object, Reference};

pub struct Rect {
    reference: Reference,
}

impl Rect {
    pub fn new(env: &Env, x: f32, y: f32, width: f32, height: f32) -> Rect {
        unsafe {
            Rect {
                reference: env.retain(env.call_constructor(
                    "com/glacyr/polyhorn/Rect",
                    "(FFFF)V",
                    &[
                        JValue::Float(x),
                        JValue::Float(y),
                        JValue::Float(width),
                        JValue::Float(height),
                    ],
                )),
            }
        }
    }

    pub fn width(&self, env: &Env) -> f32 {
        unsafe {
            match env.call_method(self.reference.as_object(), "getWidth", "()F", &[]) {
                JValue::Float(value) => value,
                _ => unreachable!(),
            }
        }
    }

    pub fn height(&self, env: &Env) -> f32 {
        unsafe {
            match env.call_method(self.reference.as_object(), "getHeight", "()F", &[]) {
                JValue::Float(value) => value,
                _ => unreachable!(),
            }
        }
    }
}

impl Object for Rect {
    fn from_reference(reference: Reference) -> Self {
        Rect { reference }
    }

    fn as_reference(&self) -> &Reference {
        &self.reference
    }
}
