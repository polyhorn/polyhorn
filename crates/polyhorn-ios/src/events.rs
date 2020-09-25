use std::rc::Rc;

pub struct ChangeEvent<T> {
    pub value: T,
}

pub struct EventListener<T> {
    closure: Option<Rc<dyn Fn(T)>>,
}

impl<T> EventListener<T> {
    pub fn is_none(&self) -> bool {
        self.closure.is_none()
    }

    pub fn is_some(&self) -> bool {
        self.closure.is_some()
    }

    pub fn none() -> EventListener<T> {
        EventListener { closure: None }
    }

    pub fn call(&self, event: T) {
        match self.closure.as_ref() {
            Some(closure) => closure(event),
            None => {}
        }
    }
}

impl<F, T> From<F> for EventListener<T>
where
    F: Fn(T) + 'static,
{
    fn from(closure: F) -> Self {
        EventListener {
            closure: Some(Rc::new(closure)),
        }
    }
}

impl<T> Clone for EventListener<T> {
    fn clone(&self) -> Self {
        EventListener {
            closure: self.closure.clone(),
        }
    }
}

impl<T> Default for EventListener<T> {
    fn default() -> Self {
        EventListener { closure: None }
    }
}
