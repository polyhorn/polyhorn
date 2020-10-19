use polyhorn_channel::{use_channel, Sender};
use polyhorn_core::CommandBuffer;
use polyhorn_ios_sys::polykit::PLYWindow;
use polyhorn_ui::hooks::SafeAreaInsets;
use std::rc::Rc;

use crate::prelude::*;
use crate::raw::{Builtin, Container, OpaqueContainer};
use crate::Key;

impl Container for PLYWindow {
    fn mount(&mut self, child: &mut OpaqueContainer) {
        if let Some(view) = child.container().to_view() {
            self.root_view_controller().view_mut().add_subview(&view)
        }
    }

    fn unmount(&mut self) {
        // A `UIWindow` is automatically closed when its retain count drops to
        // zero.
    }

    fn to_window(&self) -> Option<PLYWindow> {
        Some(self.clone())
    }
}

impl Component for Window {
    fn render(&self, manager: &mut Manager) -> Element {
        let reference = use_reference!(manager, None);

        let insets = use_reference!(manager, SafeAreaInsets::new(20.0, 0.0, 0.0, 0.0));

        let weak_insets = insets.weak(manager);

        let mut channel: Sender<SafeAreaInsets> = use_channel!(manager, {
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

        use_layout_effect!(manager, move |link, buffer| {
            let id = match reference.apply(link, |id| id.to_owned()) {
                Some(id) => id,
                None => return,
            };

            buffer.mutate(&[id], move |containers, _| {
                let container = &mut containers[0];

                let insets = container
                    .container()
                    .to_window()
                    .unwrap()
                    .root_view_controller()
                    .view_mut()
                    .safe_area_insets();

                let _ = channel.try_send(SafeAreaInsets::new(
                    insets.top as _,
                    insets.right as _,
                    insets.bottom as _,
                    insets.left as _,
                ));
            });
        });

        Element::builtin(
            Key::new(()),
            Builtin::Window,
            Element::context(
                Key::new(()),
                Rc::new(insets.apply(manager, |insets| insets.to_owned())),
                manager.children(),
            ),
            Some(reference.weak(manager)),
        )
    }
}
