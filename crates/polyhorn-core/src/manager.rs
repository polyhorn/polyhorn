use super::hooks::{UseAsync, UseContext, UseEffect, UseReference, UseState};
use super::{Bus, Context, ContextTree, Element, Key, Link, Memory, Platform, Reference, State};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;

pub struct Manager<'a, P>
where
    P: Platform + ?Sized,
{
    compositor: &'a P::Compositor,
    bus: &'a P::Bus,
    memory: &'a mut Memory<P>,
    context: &'a ContextTree,
    children: Element<P>,
    effects: Vec<Box<dyn FnOnce(&mut P::CommandBuffer)>>,
    link: Link<P>,
}

impl<'a, P> Manager<'a, P>
where
    P: Platform + ?Sized,
{
    pub fn new(
        compositor: &'a P::Compositor,
        bus: &'a P::Bus,
        memory: &'a mut Memory<P>,
        context: &'a ContextTree,
        children: Element<P>,
        link: Link<P>,
    ) -> Manager<'a, P> {
        Manager {
            compositor,
            bus,
            memory,
            context,
            children,
            effects: vec![],
            link,
        }
    }

    pub fn compositor(&self) -> &P::Compositor {
        self.compositor
    }

    pub fn children(&mut self) -> Element<P> {
        self.children.clone()
    }

    pub(crate) fn into_effects(self) -> Vec<Box<dyn FnOnce(&mut P::CommandBuffer)>> {
        self.effects
    }
}

impl<'a, P> UseAsync for Manager<'a, P>
where
    P: Platform + ?Sized,
{
    fn use_async<F>(&mut self, key: Key, task: F)
    where
        F: Future<Output = ()> + 'static,
    {
        let bus = &mut self.bus;
        self.memory.future(key, || bus.queue(task))
    }
}

impl<'a, P> UseContext for Manager<'a, P>
where
    P: Platform + ?Sized,
{
    fn use_context<T>(&mut self) -> Option<Context<T>>
    where
        T: 'static,
    {
        self.context.get::<T>().map(|value| Context::new(&value))
    }
}

impl<'a, P> UseEffect<P> for Manager<'a, P>
where
    P: Platform + ?Sized,
{
    fn use_effect<F>(&mut self, key: Key, conditions: Option<Key>, effect: F)
    where
        F: FnOnce(&mut P::CommandBuffer) + 'static,
    {
        if let Some(conditions) = conditions {
            if !self.memory.effect(key, conditions) {
                return;
            }
        }

        self.effects.push(Box::new(effect))
    }
}

impl<'a, P> UseState for Manager<'a, P>
where
    P: Platform + ?Sized,
{
    fn use_state<S>(&mut self, key: Key, initial_value: S) -> State<S>
    where
        S: Serialize + for<'b> Deserialize<'b> + 'static,
    {
        let value = self
            .memory
            .state(key, move || Rc::new(RefCell::new(initial_value)))
            .clone()
            .downcast::<RefCell<S>>()
            .unwrap();

        State::new(&value, self.link.clone())
    }
}

impl<'a, P> UseReference for Manager<'a, P>
where
    P: Platform + ?Sized,
{
    fn use_reference<R>(&mut self, key: Key) -> Reference<R>
    where
        R: 'static,
    {
        let value = self
            .memory
            .reference(key, || Rc::new(RefCell::new(None::<R>)))
            .clone()
            .downcast::<RefCell<Option<R>>>()
            .unwrap();

        Reference::new(&value)
    }
}
