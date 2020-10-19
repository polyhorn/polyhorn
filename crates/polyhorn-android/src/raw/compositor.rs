use polyhorn_android_sys::Runnable;
use polyhorn_core::{Command, Composition};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use super::{Environment, OpaqueContainer, Platform};

/// Concrete implementation of a compositor that is responsible for adding and
/// removing native views from the native view hierarchy based on the virtual
/// representation within Polyhorn.
#[derive(Clone)]
pub struct Compositor {
    environment: Environment,
    buffer: Arc<Mutex<Composition<Platform>>>,
    counter: Arc<AtomicUsize>,
}

impl Compositor {
    /// Returns a new compositor with the given shared layouter.
    pub fn new(environment: Environment) -> Compositor {
        Compositor {
            environment,
            buffer: Arc::new(Default::default()),
            counter: Arc::new(AtomicUsize::default()),
        }
    }

    fn next_id(&mut self) -> ContainerID {
        let id = self.counter.fetch_add(1, Ordering::Relaxed);
        ContainerID(id)
    }

    pub(crate) fn track(&mut self, container: OpaqueContainer) -> ContainerID {
        let id = self.next_id();
        self.buffer.lock().unwrap().insert(id, container);
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

/// Concrete implementation of a command buffer that can buffer commands before
/// committing them to the compositor.
pub struct CommandBuffer {
    compositor: Compositor,
    commands: Vec<Command<Platform>>,
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

        let activity = self.compositor.environment.activity().clone();
        let layout_tree = self.compositor.environment.layout_tree().clone();
        let state = self.compositor.buffer.clone();

        Runnable::new(&self.compositor.environment.env(), move |env| {
            let mut environment = Environment::new(
                activity,
                unsafe { env.clone().prolong_lifetime() },
                layout_tree.clone(),
            );

            let mut state = state.lock().unwrap();

            // Apply each command to this state.
            for command in commands {
                state.process(&mut environment, command);
            }

            let mut layout_tree = layout_tree.write().unwrap();
            layout_tree.recompute_roots();
        })
        .queue(&self.compositor.environment.env());
    }
}
