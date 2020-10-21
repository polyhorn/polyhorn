use polyhorn_core::{use_channel, CommandBuffer as _};
use polyhorn_ios_sys::coregraphics::CGRect;
use polyhorn_ios_sys::polykit::{PLYCallback, PLYLayoutEvent, PLYView};
use polyhorn_ui::geometry::Size;

use crate::handles::ViewHandle;
use crate::prelude::*;
use crate::raw::{Apply, Builtin, Container, ContainerID, OpaqueContainer};
use crate::{Key, Platform, Reference};

pub enum Message {
    PointerDown,
    PointerCancel,
    PointerUp,
    Layout(CGRect),
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
pub type View = polyhorn_ui::components::View<Platform, ViewHandle>;

impl Component for View {
    fn render(&self, manager: &mut Manager) -> Element {
        let view_ref: Reference<Option<ContainerID>> = use_reference!(manager, None);
        let style = self.style;

        if let Some(reference) = self.reference.as_ref() {
            reference.replace(Some(ViewHandle {
                compositor: manager.compositor().clone(),
                container_id: view_ref.weak(manager),
            }));
        }

        let on_pointer_cancel_ref = use_reference!(manager, self.on_pointer_cancel.clone());
        on_pointer_cancel_ref.replace(manager, self.on_pointer_cancel.clone());
        let on_pointer_cancel_ref = on_pointer_cancel_ref.weak(manager);

        let on_pointer_down_ref = use_reference!(manager, self.on_pointer_down.clone());
        on_pointer_down_ref.replace(manager, self.on_pointer_down.clone());
        let on_pointer_down_ref = on_pointer_down_ref.weak(manager);

        let on_pointer_up_ref = use_reference!(manager, self.on_pointer_up.clone());
        on_pointer_up_ref.replace(manager, self.on_pointer_up.clone());
        let on_pointer_up_ref = on_pointer_up_ref.weak(manager);

        let on_layout_ref = use_reference!(manager, self.on_layout.clone());
        on_layout_ref.replace(manager, self.on_layout.clone());
        let on_layout_ref = on_layout_ref.weak(manager);

        let tx = use_channel!(manager, move |mut rx| {
            async move {
                while let Some(message) = rx.next().await {
                    match message {
                        Message::PointerCancel => {
                            on_pointer_cancel_ref.apply(|listener| listener.emit(()));
                        }
                        Message::PointerDown => {
                            on_pointer_down_ref.apply(|listener| listener.emit(()));
                        }
                        Message::PointerUp => {
                            on_pointer_up_ref.apply(|listener| listener.emit(()));
                        }
                        Message::Layout(frame) => {
                            on_layout_ref.apply(|listener| {
                                listener.emit(Size {
                                    width: frame.size.width as _,
                                    height: frame.size.height as _,
                                })
                            });
                        }
                    }
                }
            }
        });

        use_layout_effect!(manager, move |link, buffer| {
            let id = match view_ref.apply(link, |view| view.to_owned()) {
                Some(id) => id,
                None => return,
            };

            buffer.mutate(&[id], move |containers, _| {
                let container = &mut containers[0];

                let layout = match container.layout() {
                    Some(layout) => layout.clone(),
                    None => return,
                };

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

                    {
                        let mut tx = tx.clone();

                        view.set_on_layout(PLYCallback::new(move |event: PLYLayoutEvent| {
                            let _ = tx.try_send(Message::Layout(event.frame()));
                        }));
                    }
                }
            });
        });

        Element::builtin(
            Key::new(()),
            Builtin::View(self.style),
            manager.children(),
            Some(view_ref.weak(manager)),
        )
    }
}
