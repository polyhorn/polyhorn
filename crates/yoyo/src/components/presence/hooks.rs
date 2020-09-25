use polyhorn::{Manager, UseContext};

use super::Presence;

pub trait UsePresence<T>
where
    T: Clone + 'static,
{
    fn use_presence(&mut self) -> Presence<T>;
}

impl<T> UsePresence<T> for Manager<'_>
where
    T: Clone + 'static,
{
    fn use_presence(&mut self) -> Presence<T> {
        Presence {
            context: self.use_context().unwrap(),
        }
    }
}

#[macro_export]
macro_rules! use_presence {
    ($manager:expr) => {
        $crate::hooks::UsePresence::use_presence($manager)
    };
}
