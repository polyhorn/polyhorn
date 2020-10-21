use polyhorn_core::{use_channel, CommandBuffer, Sender};
use polyhorn_ios_sys::polykit::{PLYCallback, PLYViewController};
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
        let reference = use_reference!(manager, None);

        let insets = use_reference!(manager, SafeAreaInsets::new(20.0, 0.0, 0.0, 0.0));

        let weak_insets = insets.weak(manager);

        let mut safe_area_insets_channel: Sender<SafeAreaInsets> = use_channel!(manager, {
            move |mut receiver| async move {
                while let Some(message) = receiver.next().await {
                    let rerender = weak_insets
                        .apply(|insets| {
                            if *insets == message {
                                false
                            } else {
                                *insets = message;
                                true
                            }
                        })
                        .unwrap_or_default();

                    if rerender {
                        weak_insets.queue_rerender();
                    }
                }
            }
        });

        let on_dismiss_ref = use_reference!(manager, self.on_dismiss.clone());
        on_dismiss_ref.replace(manager, self.on_dismiss.clone());
        let on_dismiss_ref = on_dismiss_ref.weak(manager);
        let mut event_channel: Sender<Event> = use_channel!(manager, {
            move |mut receiver| async move {
                if let Some(_) = receiver.next().await {
                    on_dismiss_ref.apply(|on_dismiss| on_dismiss.emit(()));
                }
            }
        });

        let visible = self.visible;

        use_layout_effect!(manager, move |link, buffer| {
            let id = match reference.apply(link, |id| id.to_owned()) {
                Some(id) => id,
                None => return,
            };

            buffer.mutate(&[id], move |containers, _| {
                let container = &mut containers[0];

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
            });
        });

        Element::builtin(
            Key::new(()),
            Builtin::Modal,
            Element::context(
                Key::new(()),
                Rc::new(insets.apply(manager, |insets| insets.to_owned())),
                manager.children(),
            ),
            Some(reference.weak(manager)),
        )
    }
}
