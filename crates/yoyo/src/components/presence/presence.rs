use polyhorn::Context;

use super::{PresenceContext, SafeToRemove};

pub struct Presence<T>
where
    T: Clone + 'static,
{
    pub(super) context: Context<PresenceContext<T>>,
}

impl<T> Presence<T>
where
    T: Clone + 'static,
{
    pub fn custom(&self) -> T {
        self.context.upgrade().unwrap().custom.clone()
    }

    pub fn into_dyn(self) -> Box<dyn DynPresence> {
        Box::new(self)
    }
}

pub trait DynPresence {
    fn is_animated(&self) -> bool;
    fn is_present(&self) -> bool;
    fn safe_to_remove(&self) -> SafeToRemove;
}

impl<T> DynPresence for Presence<T>
where
    T: Clone + 'static,
{
    fn is_animated(&self) -> bool {
        self.context.upgrade().unwrap().is_animated
    }

    fn is_present(&self) -> bool {
        self.context.upgrade().unwrap().is_present
    }

    fn safe_to_remove(&self) -> SafeToRemove {
        self.context.upgrade().unwrap().safe_to_remove.clone()
    }
}
