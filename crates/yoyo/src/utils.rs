use polyhorn::AnimationHandle;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SharedAnimationHandle {
    handle: Arc<Mutex<Option<AnimationHandle>>>,
}

impl SharedAnimationHandle {
    pub fn new(handle: AnimationHandle) -> SharedAnimationHandle {
        SharedAnimationHandle {
            handle: Arc::new(Mutex::new(Some(handle))),
        }
    }

    pub fn take(self) -> Option<AnimationHandle> {
        self.handle.lock().unwrap().take()
    }
}
