use std::rc::Rc;

use super::{Instance, Link, Memory, Platform};

pub struct Effect<P>
where
    P: Platform + ?Sized,
{
    instance: Rc<Instance<P>>,
    closure: Box<dyn FnOnce(&EffectLink<P>)>,
}

impl<P> Effect<P>
where
    P: Platform + ?Sized,
{
    pub fn new<F>(instance: &Rc<Instance<P>>, closure: F) -> Effect<P>
    where
        F: FnOnce(&EffectLink<P>) + 'static,
    {
        Effect {
            instance: instance.clone(),
            closure: Box::new(closure),
        }
    }

    pub fn instance(&self) -> &Rc<Instance<P>> {
        &self.instance
    }

    pub fn invoke(self, link: &EffectLink<P>) {
        (self.closure)(link)
    }
}

pub struct LayoutEffect<P>
where
    P: Platform + ?Sized,
{
    instance: Rc<Instance<P>>,
    closure: Box<dyn FnOnce(&EffectLink<P>, &mut P::CommandBuffer)>,
}

impl<P> LayoutEffect<P>
where
    P: Platform + ?Sized,
{
    pub fn new<F>(instance: &Rc<Instance<P>>, closure: F) -> LayoutEffect<P>
    where
        F: FnOnce(&EffectLink<P>, &mut P::CommandBuffer) + 'static,
    {
        LayoutEffect {
            instance: instance.clone(),
            closure: Box::new(closure),
        }
    }

    pub fn instance(&self) -> &Rc<Instance<P>> {
        &self.instance
    }

    pub fn invoke(self, link: &EffectLink<P>, buffer: &mut P::CommandBuffer) {
        (self.closure)(link, buffer)
    }
}

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
