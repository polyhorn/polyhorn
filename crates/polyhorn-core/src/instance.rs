use super::{ContextTree, Element, Memory, Platform, Renderer};
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

pub struct Instance<P>
where
    P: Platform + ?Sized,
{
    renderer: Rc<Renderer<P>>,

    /// This field holds the ID of the container that the instance was mounted
    /// onto. This field never changes.
    container: P::ContainerID,

    /// This fields holds a reference to the corresponding branch within the
    /// context tree. This field never changes once created.
    context: Rc<ContextTree>,

    memory: RefCell<Memory<P>>,
}

impl<P> Instance<P>
where
    P: Platform + ?Sized,
{
    pub fn new(
        renderer: Rc<Renderer<P>>,
        parent: Option<Rc<Instance<P>>>,
        element: Element<P>,
        container: P::ContainerID,
    ) -> Instance<P> {
        Instance {
            renderer,
            container,
            context: parent
                .map(|parent| Rc::new(parent.context.enter()))
                .unwrap_or_default(),
            memory: RefCell::new(Memory::new(element)),
        }
    }

    pub fn renderer(&self) -> &Rc<Renderer<P>> {
        &self.renderer
    }

    pub fn container(&self) -> P::ContainerID {
        self.container
    }

    pub fn context(&self) -> &ContextTree {
        &self.context
    }

    pub fn memory_mut(&self) -> RefMut<Memory<P>> {
        self.memory.borrow_mut()
    }
}
