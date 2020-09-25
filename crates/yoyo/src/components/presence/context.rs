use polyhorn::{Reference, State};

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
    pub(super) memory: Reference<Memory>,
    pub(super) marker: State<()>,
}

impl SafeToRemove {
    pub fn invoke(&self) {
        // TODO: ugly trick to get a mutable reference to the memory.
        self.memory.clone().apply(|memory| {
            memory.remove(self.id);
        });
        self.marker.replace(());
    }
}
