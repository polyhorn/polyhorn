use jni::objects::JValue;
use jni::sys::{jobject, JNIEnv};
use jni::NativeMethod;

use super::{Env, Reference};

pub struct Runnable {
    reference: Reference,
}

struct RunnableClosure(Box<dyn FnOnce(&Env) + Send>);

impl Runnable {
    pub fn new<F>(env: &Env, closure: F) -> Runnable
    where
        F: FnOnce(&Env) + Send + 'static,
    {
        extern "C" fn main(env: *mut JNIEnv, _: jobject, data: i64) {
            unsafe {
                log::error!("Invoking closure.");
                let closure = Box::<RunnableClosure>::from_raw(data as *mut _);
                closure.0(&Env::new(env));
            }
        }

        unsafe {
            env.register_natives(
                "com/glacyr/polyhorn/Runnable",
                vec![NativeMethod {
                    name: "main".to_owned().into(),
                    sig: "(J)V".to_owned().into(),
                    fn_ptr: main as *mut _,
                }],
            );

            let closure = Box::new(RunnableClosure(Box::new(closure)));
            Runnable {
                reference: env.retain(env.call_constructor(
                    "com/glacyr/polyhorn/Runnable",
                    "(J)V",
                    &[JValue::Long(Box::into_raw(closure) as i64)],
                )),
            }
        }
    }

    pub fn queue(self, env: &Env) {
        log::error!("Queueing runnable.");
        unsafe {
            env.call_method(self.reference.as_object(), "queue", "()V", &[]);
        }
    }
}
