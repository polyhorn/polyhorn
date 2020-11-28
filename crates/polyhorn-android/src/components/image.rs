use polyhorn_android_sys::{Bitmap, BitmapFactory, Env, Rect};
use polyhorn_core::CommandBuffer;
use polyhorn_ui::assets::ImageSource;
use polyhorn_ui::geometry::{Dimension, Size};
use polyhorn_ui::styles::ViewStyle;

use crate::prelude::*;
use crate::raw::{Builtin, Container, ContainerID, Environment, OpaqueContainer};
use crate::{Key, Reference};

impl Container for polyhorn_android_sys::ImageView {
    fn mount(&mut self, child: &mut OpaqueContainer, environment: &mut Environment) {
        if let Some(view) = child.container().to_view() {
            polyhorn_android_sys::ImageView::to_view(self).add_view(environment.env(), &view)
        }
    }

    fn unmount(&mut self) {}

    fn to_view(&self) -> Option<polyhorn_android_sys::View> {
        Some(polyhorn_android_sys::ImageView::to_view(self))
    }
}

struct ConcreteImage {
    image: Option<Bitmap>,
    size: Size<f32>,
}

impl Component for Image {
    fn render(&self, manager: &mut Manager) -> Element {
        let view_ref: Reference<Option<ContainerID>> = use_reference!(manager, None);

        let image_source = self.source.clone();
        let tint_color = self.style.image.tint_color.clone();

        let image = match image_source {
            ImageSource::Asset(_) => todo!(),
            ImageSource::Bytes(bytes) => {
                let env = Env::current();
                let bitmap = BitmapFactory::decode_byte_array(&env, &bytes).unwrap();
                let size = Size::new(bitmap.width(&env) as f32, bitmap.height(&env) as f32);

                ConcreteImage {
                    image: Some(bitmap),
                    size,
                }
            }
            ImageSource::Placeholder(size) => ConcreteImage { image: None, size },
        };

        let width = Dimension::Points(image.size.width);
        let height = Dimension::Points(image.size.height);

        use_layout_effect!(manager, move |link, buffer| {
            let id = match view_ref.apply(link, |id| id.to_owned()) {
                Some(id) => id,
                None => return,
            };

            buffer.mutate(&[id], move |containers, environment| {
                let container = &mut containers[0];

                let layout = match container.layout() {
                    Some(layout) => layout.clone(),
                    None => return,
                };

                let layout = layout.current();

                if let Some(view) = container.downcast_mut::<polyhorn_android_sys::ImageView>() {
                    if let Some(image) = image.image.as_ref() {
                        if let Some(_) = tint_color {
                            todo!();
                        } else {
                            view.set_image_bitmap(environment.env(), image);
                        }
                    }

                    view.set_frame(
                        environment.env(),
                        Rect::new(
                            environment.env(),
                            layout.origin.x,
                            layout.origin.y,
                            layout.size.width,
                            layout.size.height,
                        ),
                    );
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
