use super::{
    Bus, CommandBuffer, Compositor, ContainerID, Environment, Layouter, OpaqueComponent,
    OpaqueContainer,
};

/// Non-constructable type that implements the platform trait for iOS.
pub enum Platform {}

impl polyhorn_core::Platform for Platform {
    type Bus = Bus;
    type CommandBuffer = CommandBuffer;
    type Component = OpaqueComponent;
    type Compositor = Compositor;
    type Container = OpaqueContainer;
    type ContainerID = ContainerID;
    type Environment = Environment;

    fn with_compositor<F>(container: Self::Container, task: F) -> polyhorn_core::Disposable
    where
        F: FnOnce(Self::ContainerID, Self::Compositor, Self::Bus) -> polyhorn_core::Disposable
            + Send
            + 'static,
    {
        use std::sync::{Arc, RwLock};

        let layouter = Arc::new(RwLock::new(Layouter::new()));
        let mut compositor = Compositor::new(layouter.clone());
        let id = compositor.track(container);

        std::thread::Builder::new()
            .name("com.glacyr.Polyhorn".to_owned())
            .spawn(move || {
                let mut runtime = tokio::runtime::Runtime::new().unwrap();
                runtime.block_on(async move {
                    let (bus, handler) = Bus::new();
                    let _compositor = compositor.clone();
                    let _result = task(id, compositor, bus);

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
