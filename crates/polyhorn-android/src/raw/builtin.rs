use polyhorn_android_sys::View;
use polyhorn_ui::layout::LayoutNode;

use super::{Container, Environment, OpaqueContainer, Platform};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Builtin {
    ImageView,
    KeyboardAvoidingView,
    Label,
    ScrollView,
    TextInput,
    View,
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

        let view = View::new(environment.env(), environment.activity());
        return OpaqueContainer::new(layout, None, view);
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
