use polyhorn_channel::{use_channel, Sender};
use polyhorn_core::CommandBuffer;
use polyhorn_ios_sys::polykit::{PLYCallback, PLYViewController};
use polyhorn_ui::geometry::{Dimension, Size};
use polyhorn_ui::styles::{FlexDirection, Position, ViewStyle};
use std::rc::Rc;

use crate::hooks::SafeAreaInsets;
use crate::prelude::*;
use crate::raw::{Builtin, Container, OpaqueContainer};
use crate::Key;

impl Container for PLYViewController {
    fn mount(&mut self, child: &mut OpaqueContainer) {
        if let Some(view) = child.container().to_view() {
            self.view_mut().add_subview(&view)
        }
    }

    fn unmount(&mut self) {
        self.dismiss_view_controller(false, None);
    }

    fn to_view_controller(&self) -> Option<PLYViewController> {
        Some(self.clone())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Event {
    Dismiss,
}

impl Component for Modal {
    fn render(&self, manager: &mut Manager) -> Element {
        let reference = use_reference!(manager);
        let reference_clone = reference.clone();

        let insets = use_reference!(manager);
        let marker = use_state!(manager, ());

        let mut safe_area_insets_channel: Sender<SafeAreaInsets> = use_channel!(manager, {
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

        let mut on_dismiss_ref = use_reference!(manager);
        on_dismiss_ref.replace(self.on_dismiss.clone());

        let mut event_channel: Sender<Event> = use_channel!(manager, {
            move |mut receiver| async move {
                if let Some(_) = receiver.next().await {
                    on_dismiss_ref.apply(|on_dismiss| on_dismiss.emit(()));
                }
            }
        });

        let visible = self.visible;

        use_effect!(manager, move |buffer| {
            let id = match reference_clone.as_copy() {
                Some(id) => id,
                None => return,
            };

            buffer.mutate(&[id], move |containers| {
                let container = &mut containers[0];

                let frame = match container.downcast_mut::<PLYViewController>() {
                    Some(view_controller) => {
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

                if !visible {
                    container
                        .container()
                        .to_view_controller()
                        .unwrap()
                        .dismiss_view_controller(true, None);
                }

                container
                    .container()
                    .to_view_controller()
                    .unwrap()
                    .set_on_did_disappear(&PLYCallback::new(move |_| {
                        let _ = event_channel.try_send(Event::Dismiss);
                    }));

                let insets = container
                    .container()
                    .to_view_controller()
                    .unwrap()
                    .view_mut()
                    .safe_area_insets();

                let _ = safe_area_insets_channel.try_send(SafeAreaInsets::new(
                    insets.top as _,
                    insets.right as _,
                    insets.bottom as _,
                    insets.left as _,
                ));

                layout.set_style(ViewStyle {
                    position: Position::Absolute(Default::default()),
                    flex_direction: FlexDirection::Column,
                    size: Size {
                        width: Dimension::Points(frame.size.width as _),
                        height: Dimension::Points(frame.size.height as _),
                    },
                    ..Default::default()
                });

                layout.compute(Some((frame.size.width as _, frame.size.height as _)));
            });
        });

        Element::builtin(
            Key::new(()),
            Builtin::Modal,
            Element::context(
                Key::new(()),
                Rc::new(
                    insets
                        .to_owned()
                        .unwrap_or(SafeAreaInsets::new(20.0, 0.0, 0.0, 0.0)),
                ),
                manager.children(),
            ),
            Some(reference),
        )
    }
}
