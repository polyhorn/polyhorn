use super::Trigger;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct State<T>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    data: Weak<RefCell<T>>,
    link: Rc<dyn Trigger>,
}

impl<T> State<T>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    pub fn new(value: &Rc<RefCell<T>>, link: impl Trigger + 'static) -> State<T> {
        State {
            data: Rc::downgrade(value),
            link: Rc::new(link),
        }
    }

    pub fn replace(&self, value: impl Into<T>) -> Option<T> {
        if let Some(data) = self.data.upgrade() {
            let result = std::mem::replace(&mut *data.borrow_mut(), value.into());
            self.link.trigger();
            Some(result)
        } else {
            None
        }
    }

    pub fn to_owned(&self) -> T::Owned
    where
        T: ToOwned,
    {
        self.data.upgrade().unwrap().borrow_mut().to_owned()
    }
}

impl<T> Clone for State<T>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    fn clone(&self) -> Self {
        State {
            data: self.data.clone(),
            link: self.link.clone(),
        }
    }
}
