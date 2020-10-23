use polyhorn_core::EventLoop;
use polyhorn_ui::layout::LayoutTree;

use super::{
    CommandBuffer, Compositor, ContainerID, Environment, OpaqueComponent, OpaqueContainer,
};

/// Non-constructable type that implements the platform trait for iOS.
pub enum Platform {}

impl polyhorn_core::Platform for Platform {
    type CommandBuffer = CommandBuffer;
    type Component = OpaqueComponent;
    type Compositor = Compositor;
    type Container = OpaqueContainer;
    type ContainerID = ContainerID;
    type Environment = Environment;

    fn with_compositor<F>(container: Self::Container, task: F) -> polyhorn_core::Disposable
    where
        F: FnOnce(Self::ContainerID, Self::Compositor, EventLoop) -> polyhorn_core::Disposable
            + Send
            + 'static,
    {
        use std::sync::{Arc, RwLock};

        let layout_tree = Arc::new(RwLock::new(LayoutTree::new()));
        let mut compositor = Compositor::new(layout_tree);
        let id = compositor.track(container);

        std::thread::Builder::new()
            .name("com.glacyr.Polyhorn".to_owned())
            .spawn(move || {
                let mut runtime = tokio::runtime::Runtime::new().unwrap();
                runtime.block_on(async move {
                    let (evloop, handler) = EventLoop::new();
                    let _compositor = compositor.clone();
                    let _result = task(id, compositor, evloop);

                    handler.main().await;
                })
            })
            .unwrap();

        struct Abc;

        impl Drop for Abc {
            fn drop(&mut self) {}
        }

        polyhorn_core::Disposable::new(Abc)
    }
}
