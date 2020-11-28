use polyhorn_android_sys::{ImageView, View};
use polyhorn_ui::geometry::{Dimension, Size};
use polyhorn_ui::layout::LayoutNode;
use polyhorn_ui::styles::{FlexDirection, Position, Relative, ViewStyle};

use super::{Container, Environment, OpaqueContainer, Platform};

#[derive(Clone, Debug)]
pub enum Builtin {
    ImageView(ViewStyle),
    KeyboardAvoidingView,
    Label,
    ScrollView,
    TextInput,
    View(ViewStyle),
    Window,
}

impl polyhorn_core::Builtin<Platform> for Builtin {
    fn instantiate(
        &self,
        _parent: &mut OpaqueContainer,
        environment: &mut Environment,
    ) -> OpaqueContainer {
        let layout = match self {
            Builtin::Label => LayoutNode::leaf(environment.layout_tree().clone()),
            _ => LayoutNode::new(environment.layout_tree().clone()),
        };

        match self {
            Builtin::ImageView(_) => {
                let view = ImageView::new(environment.env(), environment.activity());
                let container = OpaqueContainer::new(layout, None, view);
                container
            }
            Builtin::View(_) => {
                let view = View::new(environment.env(), environment.activity());
                let container = OpaqueContainer::new(layout, None, view);
                container
            }
            Builtin::Window => {
                environment
                    .layout_tree()
                    .write()
                    .unwrap()
                    .roots_mut()
                    .push(layout.node());

                let view = View::new(environment.env(), environment.activity());
                let container = OpaqueContainer::new(layout, None, view);
                container
            }
            _ => todo!(),
        }
    }

    fn update(&self, container: &mut OpaqueContainer, environment: &mut Environment) {
        match self {
            &Builtin::ImageView(style) => {
                container.layout().unwrap().set_style(style);
            }
            &Builtin::View(style) => {
                container.layout().unwrap().set_style(style);
            }
            &Builtin::Window => {
                let activity = environment.activity();
                let env = environment.env();
                let frame = activity.bounds(&env);

                container.layout().unwrap().set_style(ViewStyle {
                    position: Position::Relative(Relative {
                        flex_shrink: 0.0,
                        flex_grow: 0.0,
                        ..Default::default()
                    }),
                    flex_direction: FlexDirection::Column,
                    size: Size {
                        width: Dimension::Points(frame.width(&env).ceil() as _),
                        height: Dimension::Points(frame.height(&env).ceil() as _),
                    },
                    ..Default::default()
                });
            }
            _ => {}
        }
    }
}

impl Container for polyhorn_android_sys::Activity {
    fn mount(&mut self, child: &mut OpaqueContainer, environment: &mut Environment) {
        if let Some(view) = child.downcast_mut::<View>() {
            self.set_content_view(environment.env(), view);
        }
    }

    fn unmount(&mut self) {
        // The activity is never unmounted.
    }
}
