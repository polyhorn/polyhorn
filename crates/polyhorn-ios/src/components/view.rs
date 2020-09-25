use futures::channel::mpsc;
use futures::StreamExt;
use polyhorn_core::{CommandBuffer, Compositor};
use polyhorn_ios_sys::{CGRect, UICallback, UICornerRadii, UIPoint, UIView};
use polyhorn_layout as layout;

use crate::*;

pub enum Message {
    PointerDown,
    PointerCancel,
    PointerUp,
}

pub struct ViewHandle {
    container_id: Reference<ContainerID>,
    compositor: crate::compositor::Compositor,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Bounds {
    pub width: f32,
    pub height: f32,
}

impl ViewHandle {
    pub fn animate<F>(&mut self, animations: F)
    where
        F: FnOnce(&mut Animator) + Send + 'static,
    {
        // Create a new command buffer.
        let mut buffer = self.compositor.buffer();

        self.animate_with_buffer(&mut buffer, animations);

        // And finally, commit the command buffer to synchronize the mutation.
        buffer.commit();
    }

    pub fn animate_with_buffer<F>(&mut self, buffer: &mut crate::CommandBuffer, animations: F)
    where
        F: FnOnce(&mut Animator) + Send + 'static,
    {
        let container_id = match self.container_id.to_owned() {
            Some(container_id) => container_id,
            None => panic!("Can't animate view that has not yet been mounted."),
        };

        // Add a mutation of the container to the command buffer.
        buffer.mutate(&[container_id], |containers| {
            if let Some(view) = containers[0].container().to_view() {
                animations(&mut Animator::new(view));
            }
        });
    }

    pub fn size_with_buffer<F>(&mut self, buffer: &mut crate::CommandBuffer, callback: F)
    where
        F: FnOnce(Bounds) + Send + 'static,
    {
        let container_id = match self.container_id.to_owned() {
            Some(container_id) => container_id,
            None => panic!("Can't measure view that has not yet been mounted."),
        };

        buffer.mutate(&[container_id], |containers| {
            if let Some(view) = containers[0].container().to_view() {
                let frame = view.frame();
                let bounds = Bounds {
                    width: frame.size.width as _,
                    height: frame.size.height as _,
                };
                callback(bounds);
            }
        });
    }
}

#[derive(Default)]
pub struct View {
    pub style: Style,
    pub on_pointer_cancel: EventListener<()>,
    pub on_pointer_down: EventListener<()>,
    pub on_pointer_up: EventListener<()>,
    pub reference: Option<Reference<ViewHandle>>,
}

impl Container for UIView {
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

    fn to_view(&self) -> Option<UIView> {
        Some(self.clone())
    }
}

fn convert_dimension_to_ui(dimension: polyhorn_style::Dimension) -> polyhorn_ios_sys::UIDimension {
    match dimension {
        polyhorn_style::Dimension::Pixels(pixels) => polyhorn_ios_sys::UIDimension {
            kind: polyhorn_ios_sys::UIDimensionKind::Pixels,
            value: pixels as _,
        },
        polyhorn_style::Dimension::Percent(percent) => polyhorn_ios_sys::UIDimension {
            kind: polyhorn_ios_sys::UIDimensionKind::Percentage,
            value: percent as _,
        },
        _ => polyhorn_ios_sys::UIDimension {
            kind: polyhorn_ios_sys::UIDimensionKind::Pixels,
            value: 0.0,
        },
    }
}

impl Component for View {
    fn render(&self, manager: &mut Manager) -> Element {
        let view_ref: Reference<ContainerID> = use_reference!(manager);
        let view_ref_effect = view_ref.clone();
        let style = self.style.clone();

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

        match &self.reference {
            Some(reference) if reference.is_none() => {
                reference.replace(ViewHandle {
                    compositor: manager.compositor().clone(),
                    container_id: view_ref.clone(),
                });
            }
            _ => {}
        }

        use_async!(manager, async move {
            if let Some(mut rx) = rx {
                while let Some(message) = rx.next().await {
                    match message {
                        Message::PointerCancel => {
                            if let Some(on_pointer_cancel) = on_pointer_cancel_ref.to_owned() {
                                on_pointer_cancel.call(());
                            }
                        }
                        Message::PointerDown => {
                            if let Some(on_pointer_down) = on_pointer_down_ref.to_owned() {
                                on_pointer_down.call(());
                            }
                        }
                        Message::PointerUp => {
                            if let Some(on_pointer_up) = on_pointer_up_ref.to_owned() {
                                on_pointer_up.call(());
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

                layout.set_style(layout::Style {
                    size: layout::Size {
                        width: style.width,
                        height: style.height,
                    },
                    min_size: layout::Size {
                        width: style.min_width,
                        height: style.min_height,
                    },
                    max_size: layout::Size {
                        width: style.max_width,
                        height: style.max_height,
                    },
                    position: style.position,
                    flex_direction: style.flex_direction,
                    flex_basis: style.flex_basis,
                    flex_grow: style.flex_grow,
                    flex_shrink: style.flex_shrink,
                    align_items: style.align_items,
                    justify_content: style.justify_content,
                    margin: layout::Insets {
                        top: style.margin.top,
                        trailing: style.margin.trailing,
                        bottom: style.margin.bottom,
                        leading: style.margin.leading,
                    },
                    padding: layout::Insets {
                        top: style.padding.top,
                        trailing: style.padding.trailing,
                        bottom: style.padding.bottom,
                        leading: style.padding.leading,
                    },
                    ..Default::default()
                });

                if let Some(view) = container.downcast_mut::<UIView>() {
                    view.set_background_color(style.background_color.clone().into());
                    view.set_alpha(style.opacity as _);

                    let top_leading = convert_dimension_to_ui(style.border_radius.top_leading);
                    let top_trailing = convert_dimension_to_ui(style.border_radius.top_trailing);
                    let bottom_trailing =
                        convert_dimension_to_ui(style.border_radius.bottom_trailing);
                    let bottom_leading =
                        convert_dimension_to_ui(style.border_radius.bottom_leading);

                    view.set_corner_radii(UICornerRadii {
                        top_leading: UIPoint::new(top_leading as _, top_leading as _),
                        top_trailing: UIPoint::new(top_trailing as _, top_trailing as _),
                        bottom_trailing: UIPoint::new(bottom_trailing as _, bottom_trailing as _),
                        bottom_leading: UIPoint::new(bottom_leading as _, bottom_leading as _),
                    });

                    view.set_hidden(style.visibility.is_hidden());

                    view.set_transform_translation_x(style.transform_translation_x);

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

                        view.set_on_pointer_cancel(UICallback::new(move |_| {
                            let _ = tx.try_send(Message::PointerCancel);
                        }));
                    }

                    {
                        let mut tx = tx.clone();

                        view.set_on_pointer_down(UICallback::new(move |_| {
                            let _ = tx.try_send(Message::PointerDown);
                        }));
                    }

                    {
                        let mut tx = tx.clone();

                        view.set_on_pointer_up(UICallback::new(move |_| {
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
