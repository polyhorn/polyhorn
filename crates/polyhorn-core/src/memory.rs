use super::{Disposable, Element, Instance, Key, Platform};
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub struct Memory<P>
where
    P: Platform + ?Sized,
{
    element: Element<P>,

    /// This is the state of this component.
    state: HashMap<Key, Rc<dyn Any>>,

    /// This is a map of references of this component.
    references: HashMap<Key, Rc<dyn Any>>,

    /// This is a map of conditions of effects.
    effects: HashMap<Key, Key>,

    /// This is a map of edges.
    edges: HashMap<Key, Rc<Instance<P>>>,

    futures: HashMap<Key, Disposable>,
}

impl<P> Memory<P>
where
    P: Platform + ?Sized,
{
    pub fn new(element: Element<P>) -> Memory<P> {
        Memory {
            element,
            state: HashMap::new(),
            references: HashMap::new(),
            effects: HashMap::new(),
            edges: HashMap::new(),
            futures: HashMap::new(),
        }
    }

    pub fn element(&self) -> &Element<P> {
        &self.element
    }

    pub fn update(&mut self, element: Element<P>) -> Element<P> {
        std::mem::replace(&mut self.element, element)
    }

    pub fn state<F>(&mut self, key: Key, initializer: F) -> &Rc<dyn Any>
    where
        F: FnOnce() -> Rc<dyn Any>,
    {
        self.state.entry(key).or_insert_with(initializer)
    }

    pub fn reference<F>(&mut self, key: Key, initializer: F) -> &Rc<dyn Any>
    where
        F: FnOnce() -> Rc<dyn Any>,
    {
        self.references.entry(key).or_insert_with(initializer)
    }

    pub fn effect(&mut self, key: Key, conditions: Key) -> bool {
        let result = !self
            .effects
            .get(&key)
            .map(|prev| prev == &conditions)
            .unwrap_or_default();

        self.effects.insert(key, conditions);

        result
    }

    pub fn future<F>(&mut self, key: Key, initializer: F)
    where
        F: FnOnce() -> Disposable,
    {
        self.futures.entry(key).or_insert_with(initializer);
    }

    pub fn keys(&self) -> HashSet<Key> {
        self.edges.keys().cloned().collect()
    }

    pub fn edge(&self, key: &Key) -> Option<&Rc<Instance<P>>> {
        self.edges.get(key)
    }

    pub fn edges(&self) -> impl Iterator<Item = &Rc<Instance<P>>> {
        self.edges.values()
    }

    pub fn add_edge(&mut self, key: Key, instance: Rc<Instance<P>>) {
        self.edges.insert(key, instance);
    }

    pub fn remove_edge(&mut self, key: &Key) -> Option<Rc<Instance<P>>> {
        self.edges.remove(key)
    }
}
