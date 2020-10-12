use super::{ContextTree, Element, Memory, Platform, Renderer, Topology};
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static INSTANCE_ID: AtomicUsize = AtomicUsize::new(0);

pub struct Instance<P>
where
    P: Platform + ?Sized,
{
    pub(crate) id: usize,

    renderer: Rc<Renderer<P>>,

    /// This field holds the ID of the container that the instance was mounted
    /// onto. This field never changes.
    container: P::ContainerID,

    /// This fields holds a reference to the corresponding branch within the
    /// context tree. This field never changes once created.
    context: Rc<ContextTree>,

    topology: RefCell<Topology<P>>,
    memory: RefCell<Memory>,
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
            id: INSTANCE_ID.fetch_add(1, Relaxed),
            renderer,
            container,
            context: parent
                .map(|parent| Rc::new(parent.context.enter()))
                .unwrap_or_default(),
            topology: RefCell::new(Topology::new(element)),
            memory: RefCell::new(Memory::new()),
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

    pub fn topology(&self) -> Ref<Topology<P>> {
        self.topology
            .try_borrow()
            .expect("Can't borrow instance topology that is already borrowed mutably.")
    }

    pub fn topology_mut(&self) -> RefMut<Topology<P>> {
        self.topology.borrow_mut()
    }

    pub fn memory(&self) -> Ref<Memory> {
        self.memory
            .try_borrow()
            .expect("Can't borrow instance memory that is already borrowed mutably.")
    }

    pub fn memory_mut(&self) -> RefMut<Memory> {
        self.memory.borrow_mut()
    }
}
