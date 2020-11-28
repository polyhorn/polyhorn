use jni::objects::JValue;

use super::{Env, Object, Reference};

pub struct Bitmap {
    reference: Reference,
}

impl Bitmap {
    pub fn width(&self, env: &Env) -> i32 {
        unsafe {
            match env.call_method(self.reference.as_object(), "getWidth", "()I", &[]) {
                JValue::Int(value) => value,
                _ => unreachable!(),
            }
        }
    }

    pub fn height(&self, env: &Env) -> i32 {
        unsafe {
            match env.call_method(self.reference.as_object(), "getHeight", "()I", &[]) {
                JValue::Int(value) => value,
                _ => unreachable!(),
            }
        }
    }
}

impl Object for Bitmap {
    fn from_reference(reference: Reference) -> Self {
        Bitmap { reference }
    }

    fn as_reference(&self) -> &Reference {
        &self.reference
    }
}
