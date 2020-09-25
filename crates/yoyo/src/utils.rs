use derivative::Derivative;
use polyhorn_ui::animation::{Animatable, Animator};
use std::sync::{Arc, Mutex};

#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub struct SharedAnimationHandle<A>
where
    A: Animatable,
{
    handle: Arc<Mutex<Option<<A::Animator as Animator>::AnimationHandle>>>,
}

impl<A> SharedAnimationHandle<A>
where
    A: Animatable,
{
    pub fn new(handle: <A::Animator as Animator>::AnimationHandle) -> SharedAnimationHandle<A> {
        SharedAnimationHandle {
            handle: Arc::new(Mutex::new(Some(handle))),
        }
    }

    pub fn take(self) -> Option<<A::Animator as Animator>::AnimationHandle> {
        self.handle.lock().unwrap().take()
    }
}
