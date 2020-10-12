use super::{Element, Instance, Key, Platform};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub struct Topology<P>
where
    P: Platform + ?Sized,
{
    element: Element<P>,

    /// This is a map of edges.
    edges: HashMap<Key, Rc<Instance<P>>>,
}

impl<P> Topology<P>
where
    P: Platform + ?Sized,
{
    pub fn new(element: Element<P>) -> Topology<P> {
        Topology {
            element,
            edges: HashMap::new(),
        }
    }

    pub fn element(&self) -> &Element<P> {
        &self.element
    }

    pub fn update(&mut self, element: Element<P>) -> Element<P> {
        std::mem::replace(&mut self.element, element)
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
