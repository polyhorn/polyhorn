use super::{Instance, Platform};
use std::rc::Weak;

pub struct Link<P>
where
    P: Platform + ?Sized,
{
    instance: Weak<Instance<P>>,
}

impl<P> Clone for Link<P>
where
    P: Platform + ?Sized,
{
    fn clone(&self) -> Self {
        Link {
            instance: self.instance.clone(),
        }
    }
}

impl<P> Link<P>
where
    P: Platform + ?Sized,
{
    pub fn new(instance: Weak<Instance<P>>) -> Link<P> {
        Link { instance }
    }
}

pub trait Trigger: 'static {
    fn trigger(&self);
}

impl<P> Trigger for Link<P>
where
    P: Platform + ?Sized,
{
    fn trigger(&self) {
        if let Some(instance) = self.instance.clone().upgrade() {
            // TODO: we should queue a rerender in the render thread instance of
            // doing it here.
            instance.renderer().rerender(&instance);
        }
    }
}
