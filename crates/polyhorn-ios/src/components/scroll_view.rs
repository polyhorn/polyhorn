use polyhorn_core::CommandBuffer;
use polyhorn_ios_sys::coregraphics::CGRect;
use polyhorn_ios_sys::polykit::{PLYScrollView, PLYView};
use polyhorn_ui::geometry::{Dimension, Size};
use polyhorn_ui::styles::{FlexDirection, Position, ViewStyle};

use crate::prelude::*;
use crate::raw::{Apply, Builtin, Container, OpaqueContainer};
use crate::Key;

impl Container for PLYScrollView {
    fn mount(&mut self, child: &mut OpaqueContainer) {
        if let Some(view) = child.container().to_view() {
            PLYScrollView::to_view(self).add_subview(&view);
        }
    }

    fn unmount(&mut self) {
        PLYScrollView::to_view(self).remove_from_superview();
    }

    fn to_view(&self) -> Option<PLYView> {
        Some(PLYScrollView::to_view(self))
    }
}

impl Component for Scrollable {
    fn render(&self, manager: &mut Manager) -> Element {
        let view_ref = use_reference!(manager, None);

        let style = self.style;

        use_effect!(manager, move |link, buffer| {
            let id = match view_ref.apply(link, |view| view.to_owned()) {
                Some(id) => id,
                None => return,
            };

            buffer.mutate(&[id], move |containers| {
                let container = &mut containers[0];

                let layout = match container.layout() {
                    Some(layout) => layout.clone(),
                    None => return,
                };

                let content_layout = match container.content_layout() {
                    Some(layout) => layout.clone(),
                    None => return,
                };

                content_layout.set_style(ViewStyle {
                    flex_direction: FlexDirection::Column,
                    position: Position::Absolute(Default::default()),
                    min_size: Size {
                        width: Dimension::Percentage(1.0),
                        height: Dimension::Percentage(1.0),
                    },
                    max_size: Size {
                        width: Dimension::Percentage(1.0),
                        height: Dimension::Undefined,
                    },
                    ..Default::default()
                });

                layout.set_style(style.view);

                if let Some(mut view) = container.container().to_view() {
                    style.view.apply(&mut view);

                    view.set_layout(move || {
                        let current = layout.current();

                        CGRect::new(
                            current.origin.x as _,
                            current.origin.y as _,
                            current.size.width as _,
                            current.size.height as _,
                        )
                    });
                }

                if let Some(view) = container.downcast_mut::<PLYScrollView>() {
                    style.scrollable.apply(view);

                    view.set_content_layout(move || {
                        let current = content_layout.current();

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
            Builtin::ScrollView,
            manager.children(),
            Some(view_ref.weak(manager)),
        )
    }
}
