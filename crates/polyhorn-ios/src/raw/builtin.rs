use polyhorn_ios_sys::polykit::{
    PLYImageView, PLYKeyboardAvoidingView, PLYLabel, PLYScrollView, PLYTextInputView, PLYView,
    PLYViewController, PLYWindow,
};
use polyhorn_ios_sys::uikit::UIApplication;
use polyhorn_ui::geometry::{Dimension, Size};
use polyhorn_ui::layout::{LayoutNode, MeasureFunc};
use polyhorn_ui::styles::{FlexDirection, Position, Relative, ViewStyle};

use super::{Container, Environment, OpaqueContainer, Platform};

/// Defines one of the native views that bridge Polyhorn with iOS's UIKit.
#[derive(Clone, Debug)]
pub enum Builtin {
    /// Renders an image.
    ImageView(ViewStyle),

    /// Automatically adjusts its layout when the system keyboard appears,
    /// changes its dimensions or disappears.
    KeyboardAvoidingView,

    /// Renders (rich) text.
    Label(MeasureFunc),

    /// Renders a view in a system-provided modal window.
    Modal,

    /// Implements scrolling gestures to facilitate layouts that exceed screen
    /// sizes.
    ScrollView {
        /// This is the style that gets applied to the scroll view itself.
        self_style: ViewStyle,

        /// This is the style that gets applied to the content of the scroll
        /// view.
        content_style: ViewStyle,
    },

    /// Accepts user input.
    TextInput,

    /// The base component.
    View(ViewStyle),

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
    fn instantiate(
        &self,
        _parent: &mut OpaqueContainer,
        environment: &mut Environment,
    ) -> OpaqueContainer {
        let layout = match self {
            Builtin::Label(_) => LayoutNode::leaf(environment.layout_tree().clone()),
            _ => LayoutNode::new(environment.layout_tree().clone()),
        };

        let mut container = match self {
            &Builtin::ImageView(_) => OpaqueContainer::new(layout, None, PLYImageView::new()),
            Builtin::KeyboardAvoidingView => {
                layout.set_style(ViewStyle {
                    position: Position::Relative(Relative {
                        flex_grow: 1.0,
                        ..Default::default()
                    }),
                    ..Default::default()
                });
                OpaqueContainer::new(layout, None, PLYKeyboardAvoidingView::new())
            }
            Builtin::Label(_) => OpaqueContainer::new(layout, None, PLYLabel::new()),
            Builtin::Modal => {
                let view_controller = PLYViewController::new();

                OpaqueContainer::new(layout, None, view_controller)
            }
            Builtin::ScrollView { .. } => {
                let content_layout = LayoutNode::new(environment.layout_tree().clone());
                OpaqueContainer::new(layout, Some(content_layout), PLYScrollView::new())
            }
            Builtin::TextInput => {
                layout.set_style(ViewStyle {
                    size: Size {
                        width: Dimension::Percentage(1.0),
                        height: Dimension::Auto,
                    },
                    ..Default::default()
                });
                OpaqueContainer::new(layout, None, PLYTextInputView::new())
            }
            Builtin::View(_) => OpaqueContainer::new(layout, None, PLYView::new()),
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
        };

        self.update(&mut container, environment);

        container
    }

    fn update(&self, container: &mut OpaqueContainer, _environment: &mut Environment) {
        match self {
            &Builtin::ImageView(style) => container.layout().unwrap().set_style(style),
            Builtin::Label(measure) => container.layout().unwrap().set_measure(measure.clone()),
            Builtin::Modal => {
                let view_controller =
                    if let Some(view_controller) = container.downcast_mut::<PLYViewController>() {
                        view_controller
                    } else {
                        return;
                    };

                let view = view_controller.view_mut();
                view.set_needs_layout();
                let frame = view.frame();

                container.layout().unwrap().set_style(ViewStyle {
                    position: Position::Absolute(Default::default()),
                    flex_direction: FlexDirection::Column,
                    size: Size {
                        width: Dimension::Points(frame.size.width as _),
                        height: Dimension::Points(frame.size.height as _),
                    },
                    ..Default::default()
                });
            }
            &Builtin::ScrollView {
                self_style,
                content_style,
            } => {
                container.layout().unwrap().set_style(self_style);
                container.content_layout().unwrap().set_style(content_style);
            }
            &Builtin::View(style) => container.layout().unwrap().set_style(style),
            Builtin::Window => {
                let window = if let Some(window) = container.downcast_mut::<PLYWindow>() {
                    window
                } else {
                    return;
                };

                let frame = window.root_view_controller().view_mut().frame();

                container.layout().unwrap().set_style(ViewStyle {
                    position: Position::Relative(Relative {
                        flex_shrink: 0.0,
                        flex_grow: 0.0,
                        ..Default::default()
                    }),
                    flex_direction: FlexDirection::Column,
                    size: Size {
                        width: Dimension::Points(frame.size.width as _),
                        height: Dimension::Points(frame.size.height as _),
                    },
                    ..Default::default()
                });
            }
            _ => {}
        }
    }
}
