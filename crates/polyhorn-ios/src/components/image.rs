use polyhorn_core::CommandBuffer;
use polyhorn_ios_sys::coregraphics::CGRect;
use polyhorn_ios_sys::polykit::{PLYImageView, PLYView};
use polyhorn_ios_sys::uikit::UIImage;
use polyhorn_ui::assets::{Asset, ImageSource};
use polyhorn_ui::geometry::{Dimension, Size};
use polyhorn_ui::styles::ViewStyle;

use crate::prelude::*;
use crate::raw::{Builtin, Container, ContainerID, Convert, OpaqueContainer};
use crate::{Key, Reference};

impl Container for PLYImageView {
    fn mount(&mut self, child: &mut OpaqueContainer) {
        if let Some(view) = child.container().to_view() {
            PLYImageView::to_view(self).add_subview(&view)
        }
    }

    fn unmount(&mut self) {
        PLYImageView::to_view(self).remove_from_superview();
    }

    fn to_view(&self) -> Option<PLYView> {
        Some(PLYImageView::to_view(self))
    }
}

struct ConcreteImage {
    image: Option<UIImage>,
    size: Size<f32>,
}

impl Component for Image {
    fn render(&self, manager: &mut Manager) -> Element {
        let view_ref: Reference<Option<ContainerID>> = use_reference!(manager, None);

        let image_source = self.source.clone();
        let tint_color = self.style.image.tint_color.clone();

        let image = match image_source {
            ImageSource::Asset(asset) => {
                let path = asset.package().to_owned() + "/" + asset.name();
                let image = UIImage::with_name(&path).unwrap();
                let size = Size::new(image.size().width as f32, image.size().height as f32);

                ConcreteImage {
                    image: Some(image),
                    size,
                }
            }
            ImageSource::Placeholder(size) => ConcreteImage { image: None, size },
            _ => unimplemented!("Image sources backed by buffers are not yet implemented."),
        };

        let width = Dimension::Points(image.size.width);
        let height = Dimension::Points(image.size.height);

        use_layout_effect!(manager, move |link, buffer| {
            let id = match view_ref.apply(link, |id| id.to_owned()) {
                Some(id) => id,
                None => return,
            };

            buffer.mutate(&[id], move |containers, _| {
                let container = &mut containers[0];

                let layout = match container.layout() {
                    Some(layout) => layout.clone(),
                    None => return,
                };

                if let Some(view) = container.downcast_mut::<PLYImageView>() {
                    if let Some(image) = image.image.as_ref() {
                        view.set_image(image);
                    }

                    if let Some(tint_color) = tint_color {
                        view.set_tint_color(&tint_color.convert());
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
            Builtin::ImageView(ViewStyle {
                size: Size { width, height },
                ..Default::default()
            }),
            manager.children(),
            Some(view_ref.weak(manager)),
        )
    }
}
