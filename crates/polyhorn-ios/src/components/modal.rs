use polyhorn_channel::{use_channel, Sender};
use polyhorn_core::CommandBuffer;
use polyhorn_ios_sys as sys;
use polyhorn_layout as layout;
use std::rc::Rc;

use crate::*;

#[derive(Clone)]
pub struct Modal {
    pub visible: bool,
    pub on_dismiss: EventListener<()>,
}

impl Default for Modal {
    fn default() -> Self {
        Modal {
            visible: true,
            on_dismiss: Default::default(),
        }
    }
}

impl Container for sys::UIViewController {
    fn mount(&mut self, child: &mut OpaqueContainer) {
        if let Some(view) = child.container().to_view() {
            self.view_mut().add_subview(&view)
        }
    }

    fn unmount(&mut self) {
        self.dismiss_view_controller(false, None);
    }

    fn to_view_controller(&self) -> Option<sys::UIViewController> {
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
                    on_dismiss_ref.apply(|on_dismiss| on_dismiss.call(()));
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

                let frame = match container.downcast_mut::<sys::UIViewController>() {
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
                    .set_on_did_disappear(&sys::UICallback::new(move |_| {
                        let _ = event_channel.try_send(Event::Dismiss);
                    }));

                let insets = container
                    .container()
                    .to_view_controller()
                    .unwrap()
                    .view_mut()
                    .safe_area_insets();

                let _ = safe_area_insets_channel.try_send(SafeAreaInsets {
                    top: insets.top as _,
                    left: insets.left as _,
                    right: insets.right as _,
                    bottom: insets.bottom as _,
                });

                layout.set_style(layout::Style {
                    position: Position::Absolute,
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
            Builtin::Modal,
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
