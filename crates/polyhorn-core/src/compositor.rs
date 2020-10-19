use super::Platform;

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
