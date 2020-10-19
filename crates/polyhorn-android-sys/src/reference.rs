use jni::objects::{GlobalRef, JObject};

use super::VM;

pub struct Reference {
    pub(crate) global_ref: GlobalRef,
    pub(crate) vm: VM,
}

impl Reference {
    pub fn as_object(&self) -> JObject {
        self.global_ref.as_obj()
    }

    pub fn vm(&self) -> &VM {
        &self.vm
    }
}

impl Clone for Reference {
    fn clone(&self) -> Self {
        Reference {
            global_ref: self.global_ref.clone(),
            vm: self.vm.internal_clone(),
        }
    }
}
