use polyhorn_android_sys::{Activity, Object, Thread};
use polyhorn_core::EventLoop;
use polyhorn_ui::layout::LayoutTree;

use super::{
    AndroidLogger, CommandBuffer, Compositor, ContainerID, Environment, OpaqueComponent,
    OpaqueContainer,
};

/// Non-constructable type that implements the platform trait for Android.
pub enum Platform {}

impl polyhorn_core::Platform for Platform {
    type CommandBuffer = CommandBuffer;
    type Component = OpaqueComponent;
    type Compositor = Compositor;
    type Container = OpaqueContainer;
    type ContainerID = ContainerID;
    type Environment = Environment;

    fn with_compositor<F>(mut container: Self::Container, task: F) -> polyhorn_core::Disposable
    where
        F: FnOnce(Self::ContainerID, Self::Compositor, EventLoop) -> polyhorn_core::Disposable
            + Send
            + 'static,
    {
        use std::sync::{Arc, RwLock};

        log::set_logger(&AndroidLogger).unwrap();
        log::set_max_level(log::LevelFilter::max());

        let activity = container.downcast_mut::<Activity>().unwrap().clone();

        let vm = activity.as_reference().vm();

        let env = unsafe { vm.env().prolong_lifetime() };

        let layout_tree = Arc::new(RwLock::new(LayoutTree::new()));

        Thread::new(&env, move |env| {
            let environment =
                Environment::new(activity, unsafe { env.prolong_lifetime() }, layout_tree);

            let mut compositor = Compositor::new(environment);
            let id = compositor.track(container);

            let mut runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async move {
                let (evloop, handler) = EventLoop::new();
                let _compositor = compositor.clone();
                let _result = task(id, compositor, evloop);

                handler.main().await;
            })
        })
        .start(&env);

        struct Connection;

        impl Drop for Connection {
            fn drop(&mut self) {}
        }

        polyhorn_core::Disposable::new(Connection)
    }
}
