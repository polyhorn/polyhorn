use jni::objects::{JObject, JValue};
use jni::sys::{jobject, JNIEnv};

use super::{Context, Env, Object, Reference, View};

#[derive(Clone)]
pub struct Activity {
    reference: Reference,
}

impl Activity {
    pub unsafe fn with_env(env: *mut JNIEnv, object: jobject) -> Activity {
        Activity {
            reference: Env::new(env).retain(JObject::from(object)),
        }
    }

    pub fn set_content_view(&mut self, env: &Env, view: &View) {
        unsafe {
            env.call_method(
                self.reference.as_object(),
                "setContentView",
                "(Landroid/view/View;)V",
                &[JValue::Object(view.as_reference().as_object())],
            );
        }
    }
}

impl Object for Activity {
    fn from_reference(reference: Reference) -> Self {
        Activity { reference }
    }

    fn as_reference(&self) -> &Reference {
        &self.reference
    }
}

impl Into<Context> for &Activity {
    fn into(self) -> Context {
        Context::from_reference(self.reference.clone())
    }
}
