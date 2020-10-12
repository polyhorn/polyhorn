use std::rc::Rc;

use super::{Instance, Memory, Platform};

pub trait Link {
    type Platform: Platform + ?Sized;

    fn instance(&self) -> &Rc<Instance<Self::Platform>>;
    fn memory(&self) -> &Memory;

    fn queue_rerender(&self) {
        self.instance().renderer().queue_rerender(self.instance())
    }
}
