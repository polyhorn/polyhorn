use std::cell::RefCell;
use std::collections::HashMap;

use super::{Container, Platform};

/// A command that can be send from any thread and will be executed on the
/// main thread.
pub enum Command<P>
where
    P: Platform + ?Sized,
{
    /// Initializes the container that corresponds to the second container ID
    /// with the given initialization closure and mounts the second given
    /// container ID onto the first given container ID.
    Mount(
        P::ContainerID,
        P::ContainerID,
        Box<dyn FnOnce(&mut P::Container, &mut P::Environment) -> P::Container + Send>,
    ),

    /// Applies a closure to all containers with the given IDs.
    Mutate(
        Vec<P::ContainerID>,
        Box<dyn FnOnce(&mut [&mut P::Container], &mut P::Environment) + Send>,
    ),

    /// Unmounts a container with the given ID.
    Unmount(P::ContainerID),
}

pub trait CommandBuffer<P>
where
    P: Platform + ?Sized,
{
    /// This function initializes a new container by running the given
    /// initializer on the UI thread and mounts it to the given parent by
    /// invoking the parent's mounting function (see
    /// [Container::Mount](Container::Mount)).
    fn mount<F>(&mut self, parent_id: P::ContainerID, initializer: F) -> P::ContainerID
    where
        F: FnOnce(&mut P::Container, &mut P::Environment) -> P::Container + Send + 'static;

    fn mutate<F>(&mut self, ids: &[P::ContainerID], mutator: F)
    where
        F: FnOnce(&mut [&mut P::Container], &mut P::Environment) + Send + 'static;

    fn unmount(&mut self, id: P::ContainerID);

    fn commit(self);
}

pub trait Compositor<P>
where
    P: Platform + ?Sized,
{
    fn buffer(&self) -> P::CommandBuffer;
}

pub struct Composition<P>
where
    P: Platform + ?Sized,
{
    map: HashMap<P::ContainerID, RefCell<P::Container>>,
}

impl<P> Composition<P>
where
    P: Platform + ?Sized,
{
    pub fn insert(&mut self, id: P::ContainerID, container: P::Container) {
        self.map.insert(id, RefCell::new(container));
    }

    pub fn process(&mut self, environment: &mut P::Environment, command: Command<P>) {
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

impl<P> Default for Composition<P>
where
    P: Platform + ?Sized,
{
    fn default() -> Self {
        Composition {
            map: HashMap::new(),
        }
    }
}
