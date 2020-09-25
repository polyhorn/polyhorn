pub use polyhorn_core::{
    render, use_async, use_context, use_effect, use_id, use_reference, use_state, Context,
    ContextProvider, Key, Reference, State, UseAsync, UseContext, UseEffect, UseReference,
};
pub use polyhorn_style::{
    self as style, style, text_style, AlignItems, Dimension, FlexDirection, Insets,
    IntoDimension as _, JustifyContent, LayoutAdjustment, Point, Position, Rect, Size,
    TextAlignment, Visibility,
};

mod animation;
mod animator;
mod builtin;
mod bus;
mod color;
mod component;
mod components;
mod compositor;
mod container;
mod context;
mod environment;
mod events;
mod font;
mod image_source;
mod layout;
mod markup;

pub use animation::{Animation, Keyframe, KeyframeAnimation};
pub use animator::{AnimationHandle, Animator};
pub use builtin::Builtin;
pub use bus::Bus;
pub use color::Color;
pub use component::{Component, OpaqueComponent};
pub use components::*;
pub use compositor::{Command, CommandBuffer, Compositor, ContainerID};
pub use container::{Container, OpaqueContainer};
pub use environment::Environment;
pub use events::{ChangeEvent, EventListener};
pub use font::Font;
pub use image_source::ImageSource;
pub use layout::{Layout, Layouter};

pub mod hooks {
    pub use super::components::{SafeAreaInsets, UseSafeAreaInsets};
}

pub struct Platform;

pub type Element = polyhorn_core::Element<Platform>;
pub type Instance = polyhorn_core::Instance<Platform>;
pub type Manager<'a> = polyhorn_core::Manager<'a, Platform>;
pub type Style = polyhorn_style::Style<Platform>;
pub type TextStyle = polyhorn_style::TextStyle<Platform>;

impl polyhorn_style::Platform for Platform {
    type Color = Color;
    type Font = Font;
}

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
        F: FnOnce(
                Self::ContainerID,
                Self::Compositor,
                Self::Bus,
                Self::Environment,
            ) -> polyhorn_core::Disposable
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
                    let _result = task(id, compositor, bus, Environment::new(layouter));

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

pub mod ffi;
