use polyhorn::{Element, Key};
use std::collections::HashMap;

/// This is a monotonically increasing ID that the `AnimatePresence` component's
/// memory uses to distinguish between remounts of the same component.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ID(usize);

#[derive(Clone)]
pub struct Memory {
    counter: usize,
    keys: HashMap<Key, ID>,
    elements: Vec<(ID, Element)>,
}

impl Memory {
    /// This function returns a new memory for `AnimatePresence` components.
    pub fn new() -> Memory {
        Memory {
            counter: 0,
            keys: HashMap::new(),
            elements: Vec::new(),
        }
    }

    /// This function increments the memory's counter by one and returns the old
    /// value.
    fn next_id(&mut self) -> ID {
        let id = self.counter;
        self.counter += 1;
        ID(id)
    }

    /// This function inserts a new element into the memory.
    pub fn insert(&mut self, element: Element) -> ID {
        let id = self.next_id();
        self.keys.insert(element.key().clone(), id);
        self.elements.push((id, element));
        id
    }

    pub fn forget(&mut self, key: &Key) -> Option<ID> {
        self.keys.remove(key)
    }

    pub fn lookup(&mut self, key: &Key) -> Option<(ID, &mut Element)> {
        let query = self.keys.get(key)?;
        self.elements.iter_mut().find_map(|(id, element)| {
            if id == query {
                Some((*id, element))
            } else {
                None
            }
        })
    }

    pub fn remove(&mut self, id: ID) -> Option<Element> {
        let index = self.elements.iter().position(|item| item.0 == id);

        if let Some(index) = index {
            Some(self.elements.remove(index).1)
        } else {
            None
        }
    }

    pub fn keys(&self) -> impl Iterator<Item = &Key> {
        self.keys.keys()
    }

    pub fn elements_by_id(&self) -> impl Iterator<Item = &(ID, Element)> {
        self.elements.iter()
    }
}
