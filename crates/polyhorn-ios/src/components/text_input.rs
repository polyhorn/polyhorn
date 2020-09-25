use polyhorn_core::CommandBuffer;
use polyhorn_ios_sys::{CGRect, UITextInputView, UIView};
use polyhorn_layout as layout;

use crate::*;

#[derive(Default)]
pub struct TextInput {
    pub placeholder_style: TextStyle,
    pub placeholder: String,
}

impl Container for UITextInputView {
    fn mount(&mut self, child: &mut OpaqueContainer) {
        if let Some(view) = child.container().to_view() {
            UITextInputView::to_view(self).add_subview(&view)
        }
    }

    fn unmount(&mut self) {
        UITextInputView::to_view(self).remove_from_superview();
    }

    fn to_view(&self) -> Option<UIView> {
        Some(UITextInputView::to_view(self))
    }
}

impl Component for TextInput {
    fn render(&self, manager: &mut Manager) -> Element {
        let view_ref: Reference<ContainerID> = use_reference!(manager);
        let view_ref_effect = view_ref.clone();

        let placeholder = self.placeholder.clone();
        let placeholder_style = self.placeholder_style.clone();

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
                        width: Dimension::Percent(1.0),
                        height: Dimension::Auto,
                    },
                    ..Default::default()
                });

                assert!(container.downcast_mut::<UITextInputView>().is_some());

                if let Some(view) = container.downcast_mut::<UITextInputView>() {
                    view.set_attributed_placeholder(&markup::attributed_string(
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
            Some(view_ref),
        )
    }
}
