use super::{Component, Element, Key, Manager, Platform};
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub struct ContextTree {
    parent: Option<Rc<ContextTree>>,
    values: RefCell<HashMap<TypeId, Rc<dyn Any>>>,
}

impl ContextTree {
    /// This function creates a new context tree root.
    pub fn new() -> ContextTree {
        ContextTree {
            parent: None,
            values: RefCell::new(HashMap::new()),
        }
    }

    /// This function enters a new branch of a given context tree.
    pub fn enter(self: &Rc<Self>) -> ContextTree {
        ContextTree {
            parent: Some(self.clone()),
            values: RefCell::new(HashMap::new()),
        }
    }

    pub fn insert<T>(&self, value: Rc<T>)
    where
        T: 'static,
    {
        self.insert_raw(value);
    }

    pub fn insert_raw(&self, value: Rc<dyn Any>) {
        self.values
            .borrow_mut()
            .insert(value.as_ref().type_id(), value);
    }

    pub fn get_flat<T>(&self) -> Option<Rc<T>>
    where
        T: 'static,
    {
        let id = TypeId::of::<T>();
        let values = self.values.borrow();
        let value = values.get(&id)?;
        value.clone().downcast::<T>().ok()
    }

    pub fn get<T>(&self) -> Option<Rc<T>>
    where
        T: 'static,
    {
        self.get_flat()
            .or_else(|| self.parent.as_ref().and_then(|parent| parent.get()))
    }
}

impl Default for ContextTree {
    fn default() -> Self {
        ContextTree::new()
    }
}

pub struct Context<T> {
    current: Weak<T>,
}

impl<T> Context<T> {
    pub fn new(value: &Rc<T>) -> Context<T> {
        Context {
            current: Rc::downgrade(value),
        }
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    pub fn is_some(&self) -> bool {
        self.current.upgrade().is_some()
    }

    pub fn upgrade(&self) -> Option<Rc<T>> {
        self.current.upgrade()
    }

    pub fn to_owned(&self) -> Option<T>
    where
        T: Clone,
    {
        Some(self.upgrade()?.as_ref().to_owned())
    }
}

impl<T> Clone for Context<T> {
    fn clone(&self) -> Self {
        Context {
            current: self.current.clone(),
        }
    }
}

pub struct ContextProvider<T>
where
    T: 'static,
{
    pub value: Rc<T>,
}

impl<T> Clone for ContextProvider<T>
where
    T: 'static,
{
    fn clone(&self) -> ContextProvider<T> {
        ContextProvider {
            value: self.value.clone(),
        }
    }
}

impl<T, P> Component<P> for ContextProvider<T>
where
    T: 'static,
    P: Platform + ?Sized,
{
    fn render(&self, manager: &mut Manager<P>) -> Element<P> {
        Element::context(Key::new(()), self.value.clone(), manager.children())
    }
}
