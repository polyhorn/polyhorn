use std::hash::Hash;

use super::{CommandBuffer, Component, Compositor, Container, Disposable, EventLoop};

/// This is a platform that needs to be implemented by every render host.
pub trait Platform: 'static {
    /// This is a virtual container that renders a built-in. These containers
    /// should be thread-safe (e.g. `Send + Sync`).
    type ContainerID: Copy + Eq + Hash + Send;

    /// This is a native container that renders a built-in. For example, this can
    /// be an UIView or a div. Native containers are usually not thread-safe and
    /// reside only on the main thread.
    type Container: Container<Self>;

    type Component: Component<Self>;

    type Compositor: Compositor<Self>;

    type CommandBuffer: CommandBuffer<Self>;

    type Environment;

    fn with_compositor<F>(container: Self::Container, task: F) -> Disposable
    where
        F: FnOnce(Self::ContainerID, Self::Compositor, EventLoop) -> Disposable + Send + 'static;
}
