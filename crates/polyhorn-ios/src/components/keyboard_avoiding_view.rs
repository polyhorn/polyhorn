use polyhorn_core::CommandBuffer;
use polyhorn_ios_sys::foundation::NSNumber;
use polyhorn_ios_sys::{CGRect, UICallback, UIKeyboardAvoidingView, UIView};
use polyhorn_layout as layout;
use std::sync::Arc;

use crate::*;

pub struct KeyboardAvoidingView {
    pub transform: Arc<dyn Fn(f32) -> LayoutAdjustment + Send + Sync>,
}

impl Container for UIKeyboardAvoidingView {
    fn mount(&mut self, child: &mut OpaqueContainer) {
        if let Some(view) = child.container().to_view() {
            UIKeyboardAvoidingView::to_view(self).add_subview(&view)
        }
    }

    fn unmount(&mut self) {
        UIKeyboardAvoidingView::to_view(self).remove_from_superview();
    }

    fn to_view(&self) -> Option<UIView> {
        Some(UIKeyboardAvoidingView::to_view(self))
    }
}

impl Component for KeyboardAvoidingView {
    fn render(&self, manager: &mut Manager) -> Element {
        let view_ref: Reference<ContainerID> = use_reference!(manager);
        let view_ref_effect = view_ref.clone();

        let is_subsequent_render = use_reference!(manager);

        let transform = self.transform.clone();

        use_effect!(manager, move |buffer| {
            let id = match view_ref_effect.as_copy() {
                Some(id) => id,
                None => return,
            };

            let is_first_render = is_subsequent_render.is_none();
            is_subsequent_render.replace(true);

            buffer.mutate(&[id], move |containers| {
                let container = &mut containers[0];

                let layout = match container.layout() {
                    Some(layout) => layout.clone(),
                    None => return,
                };

                if is_first_render {
                    layout.set_style(layout::Style {
                        flex_grow: 1.0,
                        ..Default::default()
                    });
                }

                if let Some(view) = container.downcast_mut::<UIKeyboardAvoidingView>() {
                    {
                        let layout = layout.clone();

                        view.set_on_keyboard(UICallback::new(move |height: NSNumber| {
                            let adjustment = transform(height.float_value());

                            layout.set_style(layout::Style {
                                flex_grow: 1.0,
                                margin: layout::Insets {
                                    bottom: adjustment.margin.bottom,
                                    ..Default::default()
                                },
                                ..Default::default()
                            });

                            layout.layouter().write().unwrap().recompute_roots();
                        }));
                    }

                    view.to_view().set_layout(move || {
                        let current = layout.current();

                        CGRect::new(
                            current.origin.x as _,
                            current.origin.y as _,
                            current.size.width as _,
                            current.size.height as _,
                        )
                    });
                }
            });
        });

        Element::builtin(
            Key::new(()),
            Builtin::KeyboardAvoidingView,
            manager.children(),
            Some(view_ref),
        )
    }
}
