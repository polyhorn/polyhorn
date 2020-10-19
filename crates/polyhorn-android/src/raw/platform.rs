use super::{
    AndroidLogger, Bus, CommandBuffer, Compositor, ContainerID, Environment, Layouter,
    OpaqueComponent, OpaqueContainer,
};

use polyhorn_android_sys::{Activity, Object, Thread};

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

    fn with_compositor<F>(mut container: Self::Container, task: F) -> polyhorn_core::Disposable
    where
        F: FnOnce(Self::ContainerID, Self::Compositor, Self::Bus) -> polyhorn_core::Disposable
            + Send
            + 'static,
    {
        use std::sync::{Arc, RwLock};

        log::set_logger(&AndroidLogger).unwrap();
        log::set_max_level(log::LevelFilter::max());

        let activity = container.downcast_mut::<Activity>().unwrap().clone();

        let vm = activity.as_reference().vm();

        let env = unsafe { vm.env().prolong_lifetime() };

        let layouter = Arc::new(RwLock::new(Layouter::new()));

        log::trace!("Starting Polyhorn thread ...");

        Thread::new(&env, move |env| {
            let environment =
                Environment::new(activity, unsafe { env.prolong_lifetime() }, layouter);

            let mut compositor = Compositor::new(environment);
            let id = compositor.track(container);

            let mut runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async move {
                let (bus, handler) = Bus::new();
                let _compositor = compositor.clone();
                let _result = task(id, compositor, bus);

                handler.main().await;
            })
        })
        .start(&env);

        // std::thread::Builder::new()
        //     .name("com.glacyr.Polyhorn".to_owned())
        //     .spawn(move || {
        //     })
        //     .unwrap();

        struct Connection;

        impl Drop for Connection {
            fn drop(&mut self) {}
        }

        polyhorn_core::Disposable::new(Connection)
    }
}
