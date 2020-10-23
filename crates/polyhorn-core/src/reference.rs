use std::cell::{Ref, RefMut};
use std::marker::PhantomData;

use super::{Link, Weak, WeakReference};

pub struct Reference<T> {
    instance_id: usize,
    reference_id: usize,
    marker: PhantomData<T>,
}

impl<T> Reference<T>
where
    T: 'static,
{
    pub(crate) fn new(instance_id: usize, reference_id: usize) -> Reference<T> {
        Reference {
            instance_id,
            reference_id,
            marker: PhantomData,
        }
    }

    pub fn get<'a, L>(&self, link: &'a L) -> Ref<'a, T>
    where
        L: Link,
    {
        assert_eq!(self.instance_id, link.instance().id);

        Ref::map(link.memory().reference(self.reference_id), |reference| {
            reference.downcast_ref().unwrap()
        })
    }

    pub fn replace<L>(&self, link: &L, value: T) -> T
    where
        L: Link,
    {
        assert_eq!(self.instance_id, link.instance().id);

        let reference = link.memory().reference_mut(self.reference_id);
        let mut reference = RefMut::map(reference, |reference| reference.downcast_mut().unwrap());
        std::mem::replace(&mut reference, value)
    }

    pub fn apply<L, F, O>(&self, link: &L, op: F) -> O
    where
        L: Link,
        F: FnOnce(&mut T) -> O,
    {
        assert_eq!(self.instance_id, link.instance().id);

        let reference = link.memory().reference_mut(self.reference_id);
        let mut reference = RefMut::map(reference, |reference| reference.downcast_mut().unwrap());
        op(&mut reference)
    }

    pub fn weak<L>(self, link: &L) -> WeakReference<L::Platform, T>
    where
        L: Link,
    {
        assert_eq!(self.instance_id, link.instance().id);

        WeakReference::new(Weak::new(link.instance()), self)
    }
}

impl<T> Reference<T>
where
    T: Clone + 'static,
{
    pub fn cloned<L>(self, link: &L) -> T
    where
        L: Link,
    {
        assert_eq!(self.instance_id, link.instance().id);

        link.memory()
            .reference(self.reference_id)
            .downcast_ref::<T>()
            .unwrap()
            .clone()
    }
}

impl<T> Reference<T>
where
    T: Default + 'static,
{
    pub fn take<L>(&self, link: &L) -> T
    where
        L: Link,
        T: Default,
    {
        self.replace(link, Default::default())
    }
}

impl<T> Clone for Reference<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Reference<T> {}
