use std::rc::{Rc, Weak as WeakRc};

use super::{Instance, Link, Memory, Platform, Reference, State};

/// This is a weak reference to an instance that can be converted into a link at
/// request.
pub struct Weak<P>
where
    P: Platform + ?Sized,
{
    instance: WeakRc<Instance<P>>,
}

impl<P> Weak<P>
where
    P: Platform + ?Sized,
{
    pub fn new(instance: &Rc<Instance<P>>) -> Weak<P> {
        Weak {
            instance: Rc::downgrade(instance),
        }
    }

    pub fn with_link<F, T>(&self, op: F) -> Option<T>
    where
        F: FnOnce(&WeakLink<P>) -> T,
    {
        if let Some(instance) = self.instance.upgrade() {
            Some(op(&WeakLink {
                instance: &instance,
                memory: &instance.memory(),
            }))
        } else {
            None
        }
    }
}

impl<P> Clone for Weak<P>
where
    P: Platform + ?Sized,
{
    fn clone(&self) -> Self {
        Weak {
            instance: self.instance.clone(),
        }
    }
}

pub struct WeakLink<'a, P>
where
    P: Platform + ?Sized,
{
    instance: &'a Rc<Instance<P>>,
    memory: &'a Memory,
}

impl<'a, P> WeakLink<'a, P>
where
    P: Platform + ?Sized,
{
    pub fn new(instance: &'a Rc<Instance<P>>, memory: &'a Memory) -> WeakLink<'a, P> {
        WeakLink { instance, memory }
    }
}

impl<'a, P> Link for WeakLink<'a, P>
where
    P: Platform + ?Sized,
{
    type Platform = P;

    fn instance(&self) -> &Rc<Instance<Self::Platform>> {
        self.instance
    }

    fn memory(&self) -> &Memory {
        self.memory
    }
}

pub struct WeakReference<P, T>(Weak<P>, Reference<T>)
where
    P: Platform + ?Sized,
    T: 'static;

impl<P, T> WeakReference<P, T>
where
    P: Platform + ?Sized,
    T: 'static,
{
    pub fn new(weak: Weak<P>, reference: Reference<T>) -> WeakReference<P, T> {
        WeakReference(weak, reference)
    }

    pub fn replace(&self, value: T) -> Option<T> {
        self.0.with_link(|link| self.1.replace(link, value))
    }

    pub fn apply<F, O>(&self, op: F) -> Option<O>
    where
        F: FnOnce(&mut T) -> O,
    {
        self.0.with_link(|link| self.1.apply(link, op))
    }

    pub fn with_link<F, O>(&self, op: F) -> Option<O>
    where
        F: FnOnce(&WeakLink<P>) -> O,
    {
        self.0.with_link(op)
    }

    pub fn queue_rerender(&self) -> bool {
        self.0.with_link(|link| link.queue_rerender()).is_some()
    }
}

impl<P, T> Clone for WeakReference<P, T>
where
    P: Platform + ?Sized,
    T: 'static,
{
    fn clone(&self) -> Self {
        WeakReference(self.0.clone(), self.1)
    }
}

pub struct WeakState<P, T>(Weak<P>, State<T>)
where
    P: Platform + ?Sized,
    T: 'static;

impl<P, T> WeakState<P, T>
where
    P: Platform + ?Sized,
    T: 'static,
{
    pub fn new(weak: Weak<P>, state: State<T>) -> WeakState<P, T> {
        WeakState(weak, state)
    }

    pub fn replace(&self, value: T) -> Option<T> {
        self.0.with_link(|link| self.1.replace(link, value))
    }
}

impl<P, T> Clone for WeakState<P, T>
where
    P: Platform + ?Sized,
    T: 'static,
{
    fn clone(&self) -> Self {
        WeakState(self.0.clone(), self.1)
    }
}
