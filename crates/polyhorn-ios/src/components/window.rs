use polyhorn_channel::{use_channel, Sender};
use polyhorn_core::CommandBuffer;
use polyhorn_ios_sys as sys;
use polyhorn_layout as layout;
use std::rc::Rc;

use crate::*;

pub struct Window {}

impl Container for sys::UIWindow {
    fn mount(&mut self, child: &mut OpaqueContainer) {
        if let Some(view) = child.container().to_view() {
            self.root_view_controller().view_mut().add_subview(&view)
        }
    }

    fn unmount(&mut self) {
        // A `UIWindow` is automatically closed when its retain count drops to
        // zero.
    }

    fn to_window(&self) -> Option<sys::UIWindow> {
        Some(self.clone())
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct SafeAreaInsets {
    pub top: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
}

pub trait UseSafeAreaInsets {
    fn use_safe_area_insets(&mut self) -> SafeAreaInsets;
}

impl<T> UseSafeAreaInsets for T
where
    T: UseContext,
{
    fn use_safe_area_insets(&mut self) -> SafeAreaInsets {
        self.use_context()
            .and_then(|context| context.to_owned())
            .unwrap_or_default()
    }
}

#[macro_export]
macro_rules! use_safe_area_insets {
    ($manager:expr) => {
        $crate::hooks::UseSafeAreaInsets::use_safe_area_insets($manager)
    };
}

impl Component for Window {
    fn render(&self, manager: &mut Manager) -> Element {
        let reference = use_reference!(manager);
        let reference_clone = reference.clone();

        let insets = use_reference!(manager);
        let marker = use_state!(manager, ());

        let mut channel: Sender<SafeAreaInsets> = use_channel!(manager, {
            let insets = insets.clone();

            move |mut receiver| async move {
                while let Some(message) = receiver.next().await {
                    if insets.to_owned() == Some(message) {
                        continue;
                    }

                    insets.replace(message);
                    marker.replace(());
                }
            }
        });

        use_effect!(manager, move |buffer| {
            let id = match reference_clone.as_copy() {
                Some(id) => id,
                None => return,
            };

            buffer.mutate(&[id], move |containers| {
                let container = &mut containers[0];

                let frame = match container.downcast_mut::<sys::UIWindow>() {
                    Some(window) => {
                        let mut view_controller = window.root_view_controller();
                        let view = view_controller.view_mut();
                        view.set_needs_layout();
                        view.frame()
                    }
                    None => return,
                };

                let mut layout = match container.layout() {
                    Some(layout) => layout.clone(),
                    None => return,
                };

                let insets = container
                    .container()
                    .to_window()
                    .unwrap()
                    .root_view_controller()
                    .view_mut()
                    .safe_area_insets();

                let _ = channel.try_send(SafeAreaInsets {
                    top: insets.top as _,
                    left: insets.left as _,
                    right: insets.right as _,
                    bottom: insets.bottom as _,
                });

                layout.set_style(layout::Style {
                    flex_direction: FlexDirection::Column,
                    flex_shrink: 0.0,
                    flex_grow: 0.0,
                    size: layout::Size {
                        width: Dimension::Pixels(frame.size.width as _),
                        height: Dimension::Pixels(frame.size.height as _),
                    },
                    ..Default::default()
                });

                layout.compute(Some((frame.size.width as _, frame.size.height as _)));
            });
        });

        Element::builtin(
            Key::new(()),
            Builtin::Window,
            Element::context(
                Key::new(()),
                Rc::new(insets.to_owned().unwrap_or(SafeAreaInsets {
                    top: 20.0,
                    ..Default::default()
                })),
                manager.children(),
            ),
            Some(reference),
        )
    }
}
