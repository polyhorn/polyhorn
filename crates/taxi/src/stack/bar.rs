use polyhorn::prelude::*;
use polyhorn_ui::geometry::{ByEdge, Dimension, Size};
use polyhorn_ui::layout::LayoutAxisY;
use polyhorn_ui::styles::{Position, Relative, ViewStyle};

pub struct NavigationBar {
    pub style: ViewStyle,
}

impl Component for NavigationBar {
    fn render(&self, manager: &mut Manager) -> Element {
        let insets = use_safe_area_insets!(manager);

        let height = match self.style.size.height {
            Dimension::Auto | Dimension::Undefined => Dimension::Points(44.0),
            height => height,
        };

        let view_style = ViewStyle {
            position: Position::Relative(Relative {
                flex_shrink: 0.0,
                ..Default::default()
            }),
            background_color: self.style.background_color,
            padding: ByEdge {
                vertical: LayoutAxisY {
                    top: Dimension::Points(insets.vertical.top),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };

        let content_style = ViewStyle {
            position: Position::Relative(Relative {
                flex_shrink: 0.0,
                ..Default::default()
            }),
            size: Size {
                height,
                ..Default::default()
            },
            ..Default::default()
        };

        poly!(<View style=view_style ...>
            <View style=content_style ...>
                { manager.children() }
            </View>
        </View>)
    }
}
