use polyhorn_android_sys::{Activity, View};
use polyhorn_ui::layout::LayoutNode;
use polyhorn_ui::styles::ViewStyle;

use super::{Container, Environment, OpaqueContainer, Platform};

#[derive(Clone, Debug)]
pub enum Builtin {
    ImageView,
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
        parent: &mut OpaqueContainer,
        environment: &mut Environment,
    ) -> OpaqueContainer {
        let layout = match self {
            Builtin::Label => LayoutNode::leaf(environment.layout_tree().clone()),
            _ => LayoutNode::new(environment.layout_tree().clone()),
        };

        let view = View::new(environment.env(), environment.activity());

        match self {
            Builtin::Window => {
                if let Some(activity) = parent.downcast_mut::<Activity>() {
                    let frame = activity.bounds(environment.env());
                    log::error!("Activity frame: {:#?}", frame.width(environment.env()));
                }

                environment
                    .layout_tree()
                    .write()
                    .unwrap()
                    .roots_mut()
                    .push(layout.node())
            }
            _ => {}
        }

        let container = OpaqueContainer::new(layout, None, view);

        container
    }

    fn update(&self, container: &mut OpaqueContainer) {
        match self {
            &Builtin::View(style) => {
                container.layout().unwrap().set_style(style);
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
