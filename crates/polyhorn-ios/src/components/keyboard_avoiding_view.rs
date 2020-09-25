use polyhorn_core::CommandBuffer;
use polyhorn_ios_sys::coregraphics::CGRect;
use polyhorn_ios_sys::foundation::NSNumber;
use polyhorn_ios_sys::polykit::{PLYCallback, PLYKeyboardAvoidingView, PLYView};
use polyhorn_ui::geometry::{ByEdge, Dimension};
use polyhorn_ui::layout::LayoutAxisY;
use polyhorn_ui::styles::{Position, Relative, ViewStyle};
use std::sync::Arc;

use crate::prelude::*;
use crate::raw::{Builtin, Container, ContainerID, OpaqueContainer};
use crate::{Key, Reference};

/// Structure that contains all updates that should be made to a view's layout
/// in response to a change in the visibility or dimensions of the system's
/// virtual keyboard.
#[derive(Default)]
pub struct LayoutAdjustment {
    margin: ByEdge<Dimension<f32>>,
}

impl LayoutAdjustment {
    /// Returns a new and empty layout adjustment.
    pub fn new() -> LayoutAdjustment {
        Default::default()
    }

    /// Sets the margin bottom of this layout adjustment. Note that this will
    /// be added to the existing margin of the view.
    pub fn margin_bottom(self, bottom: Dimension<f32>) -> LayoutAdjustment {
        LayoutAdjustment {
            margin: ByEdge {
                vertical: LayoutAxisY {
                    bottom,
                    ..self.margin.vertical
                },
                ..self.margin
            },
            ..self
        }
    }
}

/// A view that automatically adjusts its layout when the system keyboard
/// appears, changes its dimensions or disappears.
pub struct KeyboardAvoidingView {
    /// Transformation function that should return a adjustment based on the
    /// keyboard's height, that we apply to the layout of this view.
    pub transform: Arc<dyn Fn(f32) -> LayoutAdjustment + Send + Sync>,
}

impl Container for PLYKeyboardAvoidingView {
    fn mount(&mut self, child: &mut OpaqueContainer) {
        if let Some(view) = child.container().to_view() {
            PLYKeyboardAvoidingView::to_view(self).add_subview(&view)
        }
    }

    fn unmount(&mut self) {
        PLYKeyboardAvoidingView::to_view(self).remove_from_superview();
    }

    fn to_view(&self) -> Option<PLYView> {
        Some(PLYKeyboardAvoidingView::to_view(self))
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
                    layout.set_style(ViewStyle {
                        position: Position::Relative(Relative {
                            flex_grow: 1.0,
                            ..Default::default()
                        }),
                        ..Default::default()
                    });
                }

                if let Some(view) = container.downcast_mut::<PLYKeyboardAvoidingView>() {
                    {
                        let layout = layout.clone();

                        view.set_on_keyboard(PLYCallback::new(move |height: NSNumber| {
                            let adjustment = transform(height.float_value());

                            layout.set_style(ViewStyle {
                                position: Position::Relative(Relative {
                                    flex_grow: 1.0,
                                    ..Default::default()
                                }),
                                margin: ByEdge {
                                    vertical: LayoutAxisY {
                                        bottom: adjustment.margin.vertical.bottom,
                                        ..Default::default()
                                    },
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
