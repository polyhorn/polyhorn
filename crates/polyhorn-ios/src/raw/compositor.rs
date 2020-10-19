use dispatch::Queue;
use polyhorn_core::Container;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

use super::{Environment, Layouter, OpaqueContainer, Platform, QueueBound};

#[derive(Default)]
pub struct Composition {
    map: HashMap<ContainerID, RefCell<OpaqueContainer>>,
}

impl Composition {
    pub fn process(&mut self, environment: &mut Environment, command: Command) {
        match command {
            Command::Mount(id, parent_id, initializer) => {
                let container = if let Some(parent) = self.map.get_mut(&parent_id) {
                    let mut parent = parent.borrow_mut();
                    let mut container = initializer(&mut *parent, environment);
                    parent.mount(&mut container, environment);
                    container
                } else {
                    return;
                };

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

                    mutation(containers.as_mut_slice(), environment);
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

/// Concrete implementation of a compositor that is responsible for adding and
/// removing native views from the native view hierarchy based on the virtual
/// representation within Polyhorn.
#[derive(Clone)]
pub struct Compositor {
    buffer: Arc<QueueBound<Composition>>,
    counter: Arc<AtomicUsize>,
    layouter: Arc<RwLock<Layouter>>,
}

impl Compositor {
    /// Returns a new compositor with the given shared layouter.
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
    fn buffer(&self) -> CommandBuffer {
        CommandBuffer {
            compositor: self.clone(),
            commands: vec![],
        }
    }
}

/// An opaque ID for containers that can be shared between threads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ContainerID(usize);

/// A command that can be send from any thread and will be executed on the
/// main thread.
pub enum Command {
    /// Initializes the container that corresponds to the second container ID
    /// with the given initialization closure and mounts the second given
    /// container ID onto the first given container ID.
    Mount(
        ContainerID,
        ContainerID,
        Box<dyn FnOnce(&mut OpaqueContainer, &mut Environment) -> OpaqueContainer + Send>,
    ),

    /// Applies a closure to all containers with the given IDs.
    Mutate(
        Vec<ContainerID>,
        Box<dyn FnOnce(&mut [&mut OpaqueContainer], &mut Environment) + Send>,
    ),

    /// Unmounts a container with the given ID.
    Unmount(ContainerID),
}

/// Concrete implementation of a command buffer that can buffer commands before
/// committing them to the compositor.
pub struct CommandBuffer {
    compositor: Compositor,
    commands: Vec<Command>,
}

impl polyhorn_core::CommandBuffer<Platform> for CommandBuffer {
    fn mount<F>(&mut self, parent_id: ContainerID, initializer: F) -> ContainerID
    where
        F: FnOnce(&mut OpaqueContainer, &mut Environment) -> OpaqueContainer + Send + 'static,
    {
        let id = self.compositor.next_id();
        self.commands
            .push(Command::Mount(id, parent_id, Box::new(initializer)));
        id
    }

    fn mutate<F>(&mut self, ids: &[ContainerID], mutator: F)
    where
        F: FnOnce(&mut [&mut OpaqueContainer], &mut Environment) + Send + 'static,
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
            let mut environment = Environment::new(layouter.clone());
            for command in commands {
                state.process(&mut environment, command);
            }

            let mut layouter = layouter.write().unwrap();
            layouter.recompute_roots();
        });
    }
}
