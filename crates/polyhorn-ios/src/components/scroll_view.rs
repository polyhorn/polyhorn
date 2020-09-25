use polyhorn_core::CommandBuffer;
use polyhorn_ios_sys as sys;
use polyhorn_layout as layout;

use crate::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ScrollViewIndicatorStyle {
    Default,
    Black,
    White,
}

impl Default for ScrollViewIndicatorStyle {
    fn default() -> Self {
        ScrollViewIndicatorStyle::Default
    }
}

#[derive(Default)]
pub struct ScrollView {
    pub style: Style,
    pub content_insets: Insets<f32>,
    pub indicator_style: ScrollViewIndicatorStyle,
    pub indicator_insets: Insets<f32>,
}

impl Container for sys::UIScrollView {
    fn mount(&mut self, child: &mut OpaqueContainer) {
        if let Some(view) = child.container().to_view() {
            sys::UIScrollView::to_view(self).add_subview(&view);
        }
    }

    fn unmount(&mut self) {
        sys::UIScrollView::to_view(self).remove_from_superview();
    }

    fn to_view(&self) -> Option<sys::UIView> {
        Some(sys::UIScrollView::to_view(self))
    }
}

fn convert_dimension_to_ui(dimension: polyhorn_style::Dimension) -> polyhorn_ios_sys::UIDimension {
    match dimension {
        polyhorn_style::Dimension::Pixels(pixels) => polyhorn_ios_sys::UIDimension {
            kind: polyhorn_ios_sys::UIDimensionKind::Pixels,
            value: pixels as _,
        },
        polyhorn_style::Dimension::Percent(percent) => polyhorn_ios_sys::UIDimension {
            kind: polyhorn_ios_sys::UIDimensionKind::Percentage,
            value: percent as _,
        },
        _ => polyhorn_ios_sys::UIDimension {
            kind: polyhorn_ios_sys::UIDimensionKind::Pixels,
            value: 0.0,
        },
    }
}

impl Component for ScrollView {
    fn render(&self, manager: &mut Manager) -> Element {
        let view_ref = use_reference!(manager);

        let view_ref_effect = view_ref.clone();
        let style = self.style.clone();

        let content_insets = self.content_insets;
        let indicator_style = self.indicator_style;
        let indicator_insets = self.indicator_insets;

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

                let content_layout = match container.content_layout() {
                    Some(layout) => layout.clone(),
                    None => return,
                };

                content_layout.set_style(layout::Style {
                    flex_direction: FlexDirection::Column,
                    position: Position::Absolute,
                    min_size: layout::Size {
                        width: Dimension::Percent(1.0),
                        height: Dimension::Percent(1.0),
                    },
                    max_size: layout::Size {
                        width: Dimension::Percent(1.0),
                        height: Dimension::Undefined,
                    },
                    ..Default::default()
                });

                layout.set_style(layout::Style {
                    size: layout::Size {
                        width: style.width,
                        height: style.height,
                    },
                    flex_grow: style.flex_grow,
                    flex_shrink: style.flex_shrink,
                    ..Default::default()
                });

                if let Some(mut view) = container.container().to_view() {
                    view.set_background_color(style.background_color.clone().into());
                    view.set_alpha(style.opacity as _);

                    let top_leading = convert_dimension_to_ui(style.border_radius.top_leading);
                    let top_trailing = convert_dimension_to_ui(style.border_radius.top_trailing);
                    let bottom_trailing =
                        convert_dimension_to_ui(style.border_radius.bottom_trailing);
                    let bottom_leading =
                        convert_dimension_to_ui(style.border_radius.bottom_leading);

                    view.set_corner_radii(sys::UICornerRadii {
                        top_leading: sys::UIPoint::new(top_leading as _, top_leading as _),
                        top_trailing: sys::UIPoint::new(top_trailing as _, top_trailing as _),
                        bottom_trailing: sys::UIPoint::new(
                            bottom_trailing as _,
                            bottom_trailing as _,
                        ),
                        bottom_leading: sys::UIPoint::new(bottom_leading as _, bottom_leading as _),
                    });

                    view.set_layout(move || {
                        let current = layout.current();

                        sys::CGRect::new(
                            current.origin.x as _,
                            current.origin.y as _,
                            current.size.width as _,
                            current.size.height as _,
                        )
                    });
                }

                if let Some(view) = container.downcast_mut::<sys::UIScrollView>() {
                    // TODO: figure out if we need to adjust this mapping based
                    // on the locale / layout direction.
                    view.set_content_inset(sys::UIEdgeInsets {
                        top: content_insets.top as _,
                        left: content_insets.leading as _,
                        bottom: content_insets.bottom as _,
                        right: content_insets.trailing as _,
                    });

                    view.set_indicator_style(match indicator_style {
                        ScrollViewIndicatorStyle::Default => {
                            sys::UIScrollViewIndicatorStyle::Default
                        }
                        ScrollViewIndicatorStyle::Black => sys::UIScrollViewIndicatorStyle::Black,
                        ScrollViewIndicatorStyle::White => sys::UIScrollViewIndicatorStyle::White,
                    });

                    // TODO: figure out if we need to adjust this mapping based
                    // on the locale / layout direction.
                    view.set_scroll_indicator_insets(sys::UIEdgeInsets {
                        top: indicator_insets.top as _,
                        left: indicator_insets.leading as _,
                        bottom: indicator_insets.bottom as _,
                        right: indicator_insets.trailing as _,
                    });

                    view.set_content_layout(move || {
                        let current = content_layout.current();

                        sys::CGRect::new(
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
            Some(view_ref),
        )
    }
}
