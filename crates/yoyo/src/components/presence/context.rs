use polyhorn::{Link, WeakReference};

use super::{Memory, ID};

pub struct PresenceContext<T>
where
    T: Clone + 'static,
{
    pub custom: T,
    pub is_animated: bool,
    pub is_present: bool,
    pub safe_to_remove: SafeToRemove,
}

#[derive(Clone)]
pub struct SafeToRemove {
    pub(super) id: ID,
    pub(super) memory: WeakReference<Memory>,
}

impl SafeToRemove {
    pub fn invoke(&self) {
        self.memory.apply(|memory| memory.remove(self.id));
        self.memory.with_link(|link| link.queue_rerender());
    }
}
