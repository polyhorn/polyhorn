use polyhorn_android_sys::Rect;
use polyhorn_core::CommandBuffer;
use polyhorn_ui::geometry::Size;
use polyhorn_ui::layout::LayoutGuide;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::prelude::*;
use crate::raw::{Builtin, Container, Environment, OpaqueContainer, Platform};
use crate::{Component, Key};

pub struct AnimationHandle;

impl std::future::Future for AnimationHandle {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

impl polyhorn_ui::animation::AnimationHandle for AnimationHandle {}

pub struct Animator;

impl polyhorn_ui::animation::Animator for Animator {
    type AnimationHandle = AnimationHandle;

    fn start(&mut self, _animation: polyhorn_ui::animation::Animation) -> Self::AnimationHandle {
        todo!()
    }
}

pub struct ViewHandle;

impl polyhorn_ui::animation::Animatable for ViewHandle {
    type Animator = Animator;
    type CommandBuffer = ();

    fn animate<F>(&mut self, _animations: F)
    where
        F: FnOnce(&mut Self::Animator) + Send + 'static,
    {
        todo!()
    }

    fn animate_with_buffer<F>(&mut self, _buffer: &mut Self::CommandBuffer, _animations: F)
    where
        F: FnOnce(&mut Self::Animator) + Send + 'static,
    {
        todo!()
    }
}

impl polyhorn_ui::handles::ViewHandle for ViewHandle {
    fn layout_guide(&self) -> LayoutGuide<f32> {
        todo!()
    }

    fn size<F>(&self, _callback: F)
    where
        F: FnOnce(Size<f32>) + Send + 'static,
    {
        todo!()
    }

    fn size_with_buffer<F>(&self, _buffer: &mut Self::CommandBuffer, _callback: F)
    where
        F: FnOnce(Size<f32>) + Send + 'static,
    {
        todo!()
    }
}

impl Container for polyhorn_android_sys::View {
    fn mount(&mut self, child: &mut OpaqueContainer, environment: &mut Environment) {
        if let Some(view) = child.container().to_view() {
            self.add_view(environment.env(), &view)
        }
    }

    fn unmount(&mut self) {}

    fn to_view(&self) -> Option<polyhorn_android_sys::View> {
        Some(self.clone())
    }
}

pub type View = polyhorn_ui::components::View<Platform, ViewHandle>;

impl Component for View {
    fn render(&self, manager: &mut Manager) -> Element {
        let reference = use_reference!(manager, None);

        use_layout_effect!(manager, move |link, buffer| {
            if let Some(view) = reference.apply(link, |&mut id| id) {
                buffer.mutate(&[view], |views, environment| {
                    let layout = views[0].layout().unwrap().current();

                    if let Some(view) = views[0].downcast_mut::<polyhorn_android_sys::View>() {
                        view.set_background_color(environment.env(), 0, 255, 0, 1.0);
                        view.set_frame(
                            environment.env(),
                            Rect::new(
                                environment.env(),
                                layout.origin.x,
                                layout.origin.y,
                                layout.size.width,
                                layout.size.height,
                            ),
                        );
                    }
                });
            }
        });

        Element::builtin(
            Key::new(()),
            Builtin::View(self.style),
            manager.children(),
            Some(reference.weak(manager)),
        )
    }
}
