use std::rc::Rc;

use super::{Instance, Link, Memory, Platform};

pub struct EffectLink<'a, P>
where
    P: Platform + ?Sized,
{
    instance: &'a Rc<Instance<P>>,
    memory: &'a Memory,
}

impl<'a, P> EffectLink<'a, P>
where
    P: Platform + ?Sized,
{
    pub fn new(instance: &'a Rc<Instance<P>>, memory: &'a Memory) -> EffectLink<'a, P> {
        EffectLink { instance, memory }
    }
}

impl<'a, P> Link for EffectLink<'a, P>
where
    P: Platform + ?Sized,
{
    type Platform = P;

    fn instance(&self) -> &Rc<Instance<P>> {
        self.instance
    }

    fn memory(&self) -> &Memory {
        self.memory
    }
}
