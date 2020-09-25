use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Reference<T> {
    current: Weak<RefCell<Option<T>>>,
}

impl<T> Reference<T> {
    pub fn new(value: &Rc<RefCell<Option<T>>>) -> Reference<T> {
        Reference {
            current: Rc::downgrade(value),
        }
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    pub fn is_some(&self) -> bool {
        self.current
            .upgrade()
            .map(|value| value.borrow().is_some())
            .unwrap_or_default()
    }

    pub fn as_copy(&self) -> Option<T>
    where
        T: Copy,
    {
        *self.current.upgrade().unwrap().borrow()
    }

    pub fn to_owned(&self) -> Option<T>
    where
        T: Clone,
    {
        self.current.upgrade()?.borrow().clone()
    }

    pub fn replace(&self, value: T) -> Option<T> {
        self.current.upgrade()?.borrow_mut().replace(value)
    }

    pub fn take(&self) -> Option<T> {
        self.current.upgrade()?.borrow_mut().take()
    }

    pub fn apply<F, R>(&mut self, apply: F) -> Option<R>
    where
        F: FnOnce(&mut T) -> R,
    {
        Some(apply(self.current.upgrade()?.borrow_mut().as_mut()?))
    }
}

impl<T> Clone for Reference<T> {
    fn clone(&self) -> Self {
        Reference {
            current: self.current.clone(),
        }
    }
}

impl<T> Default for Reference<T> {
    fn default() -> Self {
        Reference {
            current: Weak::new(),
        }
    }
}
