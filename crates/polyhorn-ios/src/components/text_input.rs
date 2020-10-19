use polyhorn_core::CommandBuffer;
use polyhorn_ios_sys::coregraphics::CGRect;
use polyhorn_ios_sys::polykit::{PLYTextInputView, PLYView};
use polyhorn_ui::geometry::{Dimension, Size};
use polyhorn_ui::styles::{TextStyle, ViewStyle};

use crate::prelude::*;
use crate::raw::{attributed_string, Builtin, Container, ContainerID, OpaqueContainer};
use crate::{Key, Reference};

/// Accepts user input.
#[derive(Default)]
pub struct TextInput {
    /// Placeholder string to use when the text input is empty.
    pub placeholder: String,

    /// Text style to apply to the placeholder string.
    pub placeholder_style: TextStyle,
}

impl Container for PLYTextInputView {
    fn mount(&mut self, child: &mut OpaqueContainer) {
        if let Some(view) = child.container().to_view() {
            PLYTextInputView::to_view(self).add_subview(&view)
        }
    }

    fn unmount(&mut self) {
        PLYTextInputView::to_view(self).remove_from_superview();
    }

    fn to_view(&self) -> Option<PLYView> {
        Some(PLYTextInputView::to_view(self))
    }
}

impl Component for TextInput {
    fn render(&self, manager: &mut Manager) -> Element {
        let view_ref: Reference<Option<ContainerID>> = use_reference!(manager, None);

        let placeholder = self.placeholder.clone();
        let placeholder_style = self.placeholder_style.clone();

        use_layout_effect!(manager, move |link, buffer| {
            let id = match view_ref.apply(link, |&mut id| id) {
                Some(id) => id,
                None => return,
            };

            buffer.mutate(&[id], move |containers, _| {
                let container = &mut containers[0];

                let layout = match container.layout() {
                    Some(layout) => layout.clone(),
                    None => return,
                };

                layout.set_style(ViewStyle {
                    size: Size {
                        width: Dimension::Percentage(1.0),
                        height: Dimension::Auto,
                    },
                    ..Default::default()
                });

                assert!(container.downcast_mut::<PLYTextInputView>().is_some());

                if let Some(view) = container.downcast_mut::<PLYTextInputView>() {
                    view.set_attributed_placeholder(&attributed_string(
                        &placeholder,
                        &placeholder_style,
                    ));

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
            Builtin::TextInput,
            manager.children(),
            Some(view_ref.weak(manager)),
        )
    }
}
