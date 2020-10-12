use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;

use super::{Disposable, Key};

pub struct Memory {
    /// This is the state of this component.
    state: Vec<Box<RefCell<dyn Any>>>,
    state_ids: HashMap<Key, usize>,

    /// This is a map of references of this component.
    references: Vec<Box<RefCell<dyn Any>>>,
    reference_ids: HashMap<Key, usize>,

    /// This is a map of conditions of effects.
    effects: HashMap<Key, Key>,

    futures: HashMap<Key, Disposable>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            state: vec![],
            state_ids: HashMap::new(),
            references: vec![],
            reference_ids: HashMap::new(),
            effects: HashMap::new(),
            futures: HashMap::new(),
        }
    }

    pub fn state(&self, id: usize) -> Ref<dyn Any> {
        self.state[id].borrow()
    }

    pub fn state_mut(&self, id: usize) -> RefMut<dyn Any> {
        self.state[id].borrow_mut()
    }

    pub fn state_id<F, T>(&mut self, key: Key, initializer: F) -> usize
    where
        F: FnOnce() -> T,
        T: 'static,
    {
        let state = &mut self.state;
        let &mut id = self.state_ids.entry(key).or_insert_with(|| {
            let value = initializer();
            state.push(Box::new(RefCell::new(value)));
            state.len() - 1
        });

        id
    }

    pub fn reference(&self, id: usize) -> Ref<dyn Any> {
        self.references[id].borrow()
    }

    pub fn reference_mut(&self, id: usize) -> RefMut<dyn Any> {
        self.references[id].borrow_mut()
    }

    pub fn reference_id<F, T>(&mut self, key: Key, initializer: F) -> usize
    where
        F: FnOnce() -> T,
        T: 'static,
    {
        let references = &mut self.references;
        let &mut id = self.reference_ids.entry(key).or_insert_with(|| {
            let value = initializer();
            references.push(Box::new(RefCell::new(value)));
            references.len() - 1
        });

        id
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
}
