use polyhorn_core::CommandBuffer;
use polyhorn_ios_sys::{CGRect, UIImageView, UIView};
use polyhorn_layout as layout;
use polyhorn_style::Dimension;

use crate::*;

pub struct Image {
    pub source: ImageSource,
    pub tint_color: Option<Color>,
}

impl Container for UIImageView {
    fn mount(&mut self, child: &mut OpaqueContainer) {
        if let Some(view) = child.container().to_view() {
            UIImageView::to_view(self).add_subview(&view)
        }
    }

    fn unmount(&mut self) {
        UIImageView::to_view(self).remove_from_superview();
    }

    fn to_view(&self) -> Option<UIView> {
        Some(UIImageView::to_view(self))
    }
}

impl Component for Image {
    fn render(&self, manager: &mut Manager) -> Element {
        let view_ref: Reference<ContainerID> = use_reference!(manager);
        let view_ref_effect = view_ref.clone();

        let image_source = self.source.clone();
        let tint_color = self.tint_color.clone();

        let width = Dimension::Pixels(image_source.width() as f32);
        let height = Dimension::Pixels(image_source.height() as f32);

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
                    size: layout::Size { width, height },
                    ..Default::default()
                });

                if let Some(view) = container.downcast_mut::<UIImageView>() {
                    view.set_image(&image_source.into());

                    if let Some(tint_color) = tint_color {
                        view.set_tint_color(&tint_color.into());
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
            Builtin::ImageView,
            manager.children(),
            Some(view_ref),
        )
    }
}
