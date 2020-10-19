use jni::objects::{JObject, JValue};
use jni::{sys, JNIEnv, JavaVM, NativeMethod};

use super::Reference;

pub trait Object {
    fn from_reference(reference: Reference) -> Self;
    fn as_reference(&self) -> &Reference;
}

pub struct VM(JavaVM);

impl VM {
    pub fn env(&self) -> Env {
        Env::from(self.0.get_env().unwrap())
    }

    pub(crate) fn internal_clone(&self) -> VM {
        unsafe { VM(JavaVM::from_raw(self.0.get_java_vm_pointer()).unwrap()) }
    }
}

#[derive(Clone)]
pub struct Env<'a> {
    env: JNIEnv<'a>,
}

impl<'a> Env<'a> {
    pub unsafe fn new(env: *mut sys::JNIEnv) -> Env<'a> {
        Env {
            env: JNIEnv::from_raw(env).unwrap(),
        }
    }

    pub unsafe fn retain(&self, object: JObject<'a>) -> Reference {
        Reference {
            global_ref: self.unwrap(self.env.new_global_ref(object)),
            vm: VM(self.env.get_java_vm().unwrap()),
        }
    }

    pub unsafe fn prolong_lifetime<'b>(&self) -> Env<'b> {
        Env {
            env: std::mem::transmute(self.env.clone()),
        }
    }

    pub unsafe fn call_constructor(
        &self,
        name: &str,
        sig: &str,
        args: &[JValue<'a>],
    ) -> JObject<'a> {
        let args = args.into_iter().map(|&arg| arg.into()).collect::<Vec<_>>();
        let class = self.unwrap(self.env.find_class(name));
        self.unwrap(self.env.new_object(class, &sig, &args))
    }

    pub unsafe fn call_method(
        &self,
        object: JObject<'a>,
        name: &str,
        sig: &str,
        args: &[JValue],
    ) -> JValue<'a> {
        let args = args.into_iter().map(|&arg| arg.into()).collect::<Vec<_>>();
        self.unwrap(self.env.call_method(object, name, sig, &args))
    }

    pub unsafe fn register_natives(
        &self,
        name: &str,
        methods: impl IntoIterator<Item = NativeMethod>,
    ) {
        let class = self.unwrap(self.env.find_class(name));
        self.unwrap(
            self.env.register_native_methods(
                class,
                &methods
                    .into_iter()
                    .map(|method| jni::NativeMethod {
                        name: method.name.into(),
                        sig: method.sig.into(),
                        fn_ptr: method.fn_ptr,
                    })
                    .collect::<Vec<_>>(),
            ),
        );
    }

    unsafe fn unwrap<T>(&self, value: jni::errors::Result<T>) -> T {
        if value.is_err() {
            let _ = self.env.exception_describe();
        }

        value.unwrap()
    }
}

impl<'a> From<JNIEnv<'a>> for Env<'a> {
    fn from(env: JNIEnv<'a>) -> Self {
        Env { env }
    }
}
