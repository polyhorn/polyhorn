use super::hooks::{UseAsync, UseContext, UseEffect, UseReference, UseState};
use super::{
    Context, ContextTree, EffectLink, Element, EventLoop, Instance, Key, Link, Memory, Platform,
    Reference, State, Weak, WeakLink,
};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::rc::Rc;

pub struct Manager<'a, P>
where
    P: Platform + ?Sized,
{
    compositor: &'a P::Compositor,
    bus: &'a EventLoop,
    memory: &'a mut Memory,
    context: &'a ContextTree,
    children: Element<P>,
    effects: Vec<Box<dyn FnOnce(&EffectLink<P>, &mut P::CommandBuffer)>>,
    instance: &'a Rc<Instance<P>>,
}

impl<'a, P> Manager<'a, P>
where
    P: Platform + ?Sized,
{
    pub fn new(
        compositor: &'a P::Compositor,
        bus: &'a EventLoop,
        memory: &'a mut Memory,
        context: &'a ContextTree,
        children: Element<P>,
        instance: &'a Rc<Instance<P>>,
    ) -> Manager<'a, P> {
        Manager {
            compositor,
            bus,
            memory,
            context,
            children,
            effects: vec![],
            instance,
        }
    }

    pub fn compositor(&self) -> &P::Compositor {
        self.compositor
    }

    pub fn children(&mut self) -> Element<P> {
        self.children.clone()
    }

    pub fn bind<F, I>(&self, closure: F) -> impl Fn(I)
    where
        F: Fn(&WeakLink<P>, I),
    {
        let weak = Weak::new(self.instance);

        move |input: I| {
            weak.with_link(|link| closure(link, input));
        }
    }

    pub(crate) fn into_effects(
        self,
    ) -> Vec<Box<dyn FnOnce(&EffectLink<P>, &mut P::CommandBuffer)>> {
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
        F: FnOnce(&EffectLink<P>, &mut P::CommandBuffer) + 'static,
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
        let state_id = self.memory.state_id(key, move || initial_value);

        State::new(self.instance.id, state_id)
    }
}

impl<'a, P> UseReference for Manager<'a, P>
where
    P: Platform + ?Sized,
{
    fn use_reference<R, I>(&mut self, key: Key, initializer: I) -> Reference<R>
    where
        R: 'static,
        I: FnOnce() -> R,
    {
        let reference_id = self.memory.reference_id(key, initializer);

        Reference::new(self.instance.id, reference_id)
    }
}

impl<'a, P> Link for Manager<'a, P>
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
