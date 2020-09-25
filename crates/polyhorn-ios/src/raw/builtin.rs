use polyhorn_ios_sys::polykit::{
    PLYImageView, PLYKeyboardAvoidingView, PLYLabel, PLYScrollView, PLYTextInputView, PLYView,
    PLYViewController, PLYWindow,
};
use polyhorn_ios_sys::uikit::UIApplication;

use super::{Container, Environment, Layout, OpaqueContainer, Platform};

/// Defines one of the native views that bridge Polyhorn with iOS's UIKit.
#[derive(Copy, Clone, Debug)]
pub enum Builtin {
    /// Renders an image.
    ImageView,

    /// Automatically adjusts its layout when the system keyboard appears,
    /// changes its dimensions or disappears.
    KeyboardAvoidingView,

    /// Renders (rich) text.
    Label,

    /// Renders a view in a system-provided modal window.
    Modal,

    /// Implements scrolling gestures to facilitate layouts that exceed screen
    /// sizes.
    ScrollView,

    /// Accepts user input.
    TextInput,

    /// The base component.
    View,

    /// The root component.
    Window,
}

impl Container for UIApplication {
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
            Builtin::ImageView => OpaqueContainer::new(layout, None, PLYImageView::new()),
            Builtin::KeyboardAvoidingView => {
                OpaqueContainer::new(layout, None, PLYKeyboardAvoidingView::new())
            }
            Builtin::Label => OpaqueContainer::new(layout, None, PLYLabel::new()),
            Builtin::Modal => OpaqueContainer::new(layout, None, PLYViewController::new()),
            Builtin::ScrollView => OpaqueContainer::new(
                layout,
                Some(Layout::new(environment.layouter().clone())),
                PLYScrollView::new(),
            ),
            Builtin::TextInput => OpaqueContainer::new(layout, None, PLYTextInputView::new()),
            Builtin::View => OpaqueContainer::new(layout, None, PLYView::new()),
            Builtin::Window => {
                let mut window = PLYWindow::new();
                window.set_root_view_controller(PLYViewController::new());

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
