use super::{Container, Environment, Layout, OpaqueContainer, Platform};
use polyhorn_ios_sys as sys;

#[derive(Copy, Clone, Debug)]
pub enum Builtin {
    KeyboardAvoidingView,
    Label,
    Modal,
    ScrollView,
    TextInput,
    ImageView,
    View,
    Window,
}

impl Container for sys::UIApplication {
    fn mount(&mut self, child: &mut OpaqueContainer) {
        let container = child.container();
        if let Some(mut window) = container.to_window() {
            window.make_key_and_visible()
        }
    }

    fn unmount(&mut self) {
        // Applications are never unmounted.
        unimplemented!("Applications cannot be unmounted.");
    }
}

impl polyhorn_core::Builtin<Platform> for Builtin {
    fn instantiate(&self, environment: &mut Environment) -> OpaqueContainer {
        let layout = match self {
            Builtin::Label => Layout::leaf(environment.layouter().clone()),
            _ => Layout::new(environment.layouter().clone()),
        };

        match self {
            Builtin::ImageView => OpaqueContainer::new(layout, None, sys::UIImageView::new()),
            Builtin::KeyboardAvoidingView => {
                OpaqueContainer::new(layout, None, sys::UIKeyboardAvoidingView::new())
            }
            Builtin::Label => OpaqueContainer::new(layout, None, sys::UILabel::new()),
            Builtin::Modal => OpaqueContainer::new(layout, None, sys::UIViewController::new()),
            Builtin::ScrollView => OpaqueContainer::new(
                layout,
                Some(Layout::new(environment.layouter().clone())),
                sys::UIScrollView::new(),
            ),
            Builtin::TextInput => OpaqueContainer::new(layout, None, sys::UITextInputView::new()),
            Builtin::View => OpaqueContainer::new(layout, None, sys::UIView::new()),
            Builtin::Window => {
                let mut window = sys::UIWindow::new();
                window.set_root_view_controller(sys::UIViewController::new());

                // TODO: where do we remove roots?
                layout
                    .layouter()
                    .write()
                    .unwrap()
                    .roots_mut()
                    .push(layout.node());

                OpaqueContainer::new(layout, None, window)
            }
        }
    }
}
