use std::cell::{Ref, RefMut};
use std::marker::PhantomData;

use super::{Link, Weak, WeakState};

pub struct State<T>
where
    T: 'static,
{
    instance_id: usize,
    state_id: usize,
    marker: PhantomData<T>,
}

impl<T> State<T> {
    pub fn new(instance_id: usize, state_id: usize) -> State<T> {
        State {
            instance_id,
            state_id,
            marker: PhantomData,
        }
    }

    pub fn get<'a, L>(&self, link: &'a L) -> Ref<'a, T>
    where
        L: Link,
    {
        assert_eq!(self.instance_id, link.instance().id);

        Ref::map(link.memory().state(self.state_id), |state| {
            state.downcast_ref().unwrap()
        })
    }

    pub fn replace<L>(&self, link: &L, value: T) -> T
    where
        L: Link,
    {
        assert_eq!(self.instance_id, link.instance().id);

        link.instance().renderer().queue_rerender(link.instance());

        let mut state = RefMut::map(link.memory().state_mut(self.state_id), |state| {
            state.downcast_mut().unwrap()
        });
        std::mem::replace(&mut state, value)
    }

    pub fn weak<L>(self, link: &L) -> WeakState<L::Platform, T>
    where
        L: Link,
    {
        assert_eq!(self.instance_id, link.instance().id);

        WeakState::new(Weak::new(link.instance()), self)
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for State<T> {}
