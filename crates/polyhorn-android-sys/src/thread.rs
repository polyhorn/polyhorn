use jni::objects::JValue;
use jni::sys;
use jni::NativeMethod;

use super::{Env, Reference};

pub struct Thread {
    reference: Reference,
}

struct ThreadClosure(Box<dyn FnOnce(&Env) + Send>);

impl Thread {
    pub fn new<F>(env: &Env, closure: F) -> Thread
    where
        F: FnOnce(&Env) + Send + 'static,
    {
        extern "C" fn main(env: *mut sys::JNIEnv, _: sys::jobject, data: i64) {
            unsafe {
                let closure = Box::<ThreadClosure>::from_raw(data as *mut _);
                closure.0(&Env::new(env));
            }
        }

        unsafe {
            env.register_natives(
                "com/glacyr/polyhorn/PolyhornThread",
                vec![NativeMethod {
                    name: "main".to_owned().into(),
                    sig: "(J)V".to_owned().into(),
                    fn_ptr: main as *mut _,
                }],
            );

            let closure = Box::new(ThreadClosure(Box::new(closure)));
            Thread {
                reference: env.retain(env.call_constructor(
                    "com/glacyr/polyhorn/PolyhornThread",
                    "(J)V",
                    &[JValue::Long(Box::into_raw(closure) as i64)],
                )),
            }
        }
    }

    pub fn start(self, env: &Env) {
        unsafe {
            env.call_method(self.reference.as_object(), "start", "()V", &[]);
        }
    }
}
