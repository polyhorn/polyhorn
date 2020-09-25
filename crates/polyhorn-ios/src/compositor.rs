use dispatch::Queue;
use polyhorn_core::Container;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

use super::{ffi::QueueBound, Layouter, OpaqueContainer, Platform};

#[derive(Default)]
pub struct Composition {
    map: HashMap<ContainerID, RefCell<OpaqueContainer>>,
}

impl Composition {
    pub fn process(&mut self, command: Command) {
        match command {
            Command::Mount(id, parent_id, initializer) => {
                let mut container = initializer();

                if let Some(parent) = self.map.get_mut(&parent_id) {
                    parent.borrow_mut().mount(&mut container);
                }

                self.map.insert(id, RefCell::new(container));
            }
            Command::Mutate(ids, mutation) => {
                let borrows = ids
                    .into_iter()
                    .map(|id| self.map.get(&id).map(|container| container.borrow_mut()))
                    .collect::<Option<Vec<_>>>();

                if let Some(mut borrows) = borrows {
                    let mut containers = borrows
                        .iter_mut()
                        .map(|borrow| &mut **borrow)
                        .collect::<Vec<_>>();

                    mutation(containers.as_mut_slice());
                }
            }
            Command::Unmount(id) => {
                if let Some(container) = self.map.remove(&id) {
                    container.borrow_mut().unmount();
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Compositor {
    buffer: Arc<QueueBound<Composition>>,
    counter: Arc<AtomicUsize>,
    layouter: Arc<RwLock<Layouter>>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ContainerID(usize);

pub enum Command {
    Mount(
        ContainerID,
        ContainerID,
        Box<dyn FnOnce() -> OpaqueContainer + Send>,
    ),
    Mutate(
        Vec<ContainerID>,
        Box<dyn FnOnce(&mut [&mut OpaqueContainer]) + Send>,
    ),
    Unmount(ContainerID),
}

pub struct CommandBuffer {
    compositor: Compositor,
    commands: Vec<Command>,
}

impl Compositor {
    pub fn new(layouter: Arc<RwLock<Layouter>>) -> Compositor {
        Compositor {
            buffer: Arc::new(QueueBound::new(Queue::main(), || Default::default())),
            counter: Arc::new(AtomicUsize::default()),
            layouter,
        }
    }

    fn next_id(&mut self) -> ContainerID {
        let id = self.counter.fetch_add(1, Ordering::Relaxed);
        ContainerID(id)
    }

    pub(crate) fn track(&mut self, container: OpaqueContainer) -> ContainerID {
        let id = self.next_id();

        unsafe {
            self.buffer.with_adopt(container, move |state, container| {
                state.map.insert(id, RefCell::new(container));
            });
        }

        id
    }
}

impl polyhorn_core::Compositor<Platform> for Compositor {
    fn buffer(&mut self) -> CommandBuffer {
        CommandBuffer {
            compositor: self.clone(),
            commands: vec![],
        }
    }
}

impl polyhorn_core::CommandBuffer<Platform> for CommandBuffer {
    fn mount<F>(&mut self, parent_id: ContainerID, initializer: F) -> ContainerID
    where
        F: FnOnce() -> OpaqueContainer + Send + 'static,
    {
        let id = self.compositor.next_id();
        self.commands
            .push(Command::Mount(id, parent_id, Box::new(initializer)));
        id
    }

    fn mutate<F>(&mut self, ids: &[ContainerID], mutator: F)
    where
        F: FnOnce(&mut [&mut OpaqueContainer]) + Send + 'static,
    {
        self.commands
            .push(Command::Mutate(ids.to_owned(), Box::new(mutator)));
    }

    fn unmount(&mut self, id: ContainerID) {
        self.commands.push(Command::Unmount(id));
    }

    fn commit(mut self) {
        let commands = std::mem::take(&mut self.commands);

        let layouter = self.compositor.layouter.clone();

        self.compositor.buffer.with(move |state| {
            // Apply each command to this state.
            for command in commands {
                state.process(command);
            }

            let mut layouter = layouter.write().unwrap();
            layouter.recompute_roots();
        });
    }
}
