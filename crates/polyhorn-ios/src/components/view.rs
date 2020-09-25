use futures::channel::mpsc;
use futures::StreamExt;
use polyhorn_core::CommandBuffer as _;
use polyhorn_ios_sys::coregraphics::CGRect;
use polyhorn_ios_sys::polykit::{PLYCallback, PLYView};

use crate::handles::ViewHandle;
use crate::prelude::*;
use crate::raw::{Apply, Builtin, Container, ContainerID, OpaqueContainer};
use crate::{Key, Reference};

pub enum Message {
    PointerDown,
    PointerCancel,
    PointerUp,
}

impl Container for PLYView {
    fn mount(&mut self, child: &mut OpaqueContainer) {
        if let Some(view) = child.container().to_view() {
            self.add_subview(&view)
        } else if let Some(view_controller) = child.container().to_view_controller() {
            self.window()
                .unwrap()
                .root_view_controller()
                .present_view_controller(&view_controller, true, None);
        }
    }

    fn unmount(&mut self) {
        self.remove_from_superview();
    }

    fn to_view(&self) -> Option<PLYView> {
        Some(self.clone())
    }
}

/// Specializes the generic View component with the iOS-specific concrete
/// view handle.
pub type View = polyhorn_ui::components::View<ViewHandle>;

impl Component for View {
    fn render(&self, manager: &mut Manager) -> Element {
        let view_ref: Reference<ContainerID> = use_reference!(manager);
        let view_ref_effect = view_ref.clone();
        let style = self.style;

        let tx = use_reference!(manager);
        let mut rx = None;

        if tx.is_none() {
            let (new_tx, new_rx) = mpsc::channel::<Message>(1024);
            rx = Some(new_rx);

            tx.replace(new_tx);
        }

        let tx = tx.to_owned().unwrap();

        let on_pointer_cancel_ref = use_reference!(manager);
        on_pointer_cancel_ref.replace(self.on_pointer_cancel.clone());

        let on_pointer_down_ref = use_reference!(manager);
        on_pointer_down_ref.replace(self.on_pointer_down.clone());

        let on_pointer_up_ref = use_reference!(manager);
        on_pointer_up_ref.replace(self.on_pointer_up.clone());

        self.reference.replace(ViewHandle {
            compositor: manager.compositor().clone(),
            container_id: view_ref.clone(),
        });

        use_async!(manager, async move {
            if let Some(mut rx) = rx {
                while let Some(message) = rx.next().await {
                    match message {
                        Message::PointerCancel => {
                            if let Some(on_pointer_cancel) = on_pointer_cancel_ref.to_owned() {
                                on_pointer_cancel.emit(());
                            }
                        }
                        Message::PointerDown => {
                            if let Some(on_pointer_down) = on_pointer_down_ref.to_owned() {
                                on_pointer_down.emit(());
                            }
                        }
                        Message::PointerUp => {
                            if let Some(on_pointer_up) = on_pointer_up_ref.to_owned() {
                                on_pointer_up.emit(());
                            }
                        }
                    }
                }
            }
        });

        use_effect!(manager, move |buffer| {
            let id = match view_ref_effect.as_copy() {
                Some(id) => id,
                None => return,
            };

            buffer.mutate(&[id], move |containers| {
                let container = &mut containers[0];

                let layout = match container.layout() {
                    Some(layout) => layout.clone(),
                    None => return,
                };

                layout.set_style(style);

                if let Some(view) = container.downcast_mut::<PLYView>() {
                    style.apply(view);

                    view.set_layout(move || {
                        let current = layout.current();

                        CGRect::new(
                            current.origin.x as _,
                            current.origin.y as _,
                            current.size.width as _,
                            current.size.height as _,
                        )
                    });

                    {
                        let mut tx = tx.clone();

                        view.set_on_pointer_cancel(PLYCallback::new(move |_| {
                            let _ = tx.try_send(Message::PointerCancel);
                        }));
                    }

                    {
                        let mut tx = tx.clone();

                        view.set_on_pointer_down(PLYCallback::new(move |_| {
                            let _ = tx.try_send(Message::PointerDown);
                        }));
                    }

                    {
                        let mut tx = tx.clone();

                        view.set_on_pointer_up(PLYCallback::new(move |_| {
                            let _ = tx.try_send(Message::PointerUp);
                        }));
                    }
                }
            });
        });

        Element::builtin(
            Key::new(()),
            Builtin::View,
            manager.children(),
            Some(view_ref),
        )
    }
}
