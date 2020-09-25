use super::{
    AlignItems, BorderRadius, Dimension, FlexDirection, Insets, JustifyContent, Margin, Padding,
    Platform, Position, Visibility,
};
use derivative::Derivative;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct LayoutAdjustment {
    pub margin: Insets<Dimension>,
}

impl LayoutAdjustment {
    pub fn new() -> LayoutAdjustment {
        Default::default()
    }

    pub fn margin_bottom(self, bottom: Dimension) -> LayoutAdjustment {
        LayoutAdjustment {
            margin: Insets {
                bottom,
                ..self.margin
            },
            ..self
        }
    }
}

#[derive(Derivative)]
#[derivative(Clone, Debug, Default)]
pub struct Style<P>
where
    P: Platform,
{
    pub background_color: P::Color,

    pub border_radius: BorderRadius,

    pub position: Position,

    #[derivative(Default(value = "1.0"))]
    pub opacity: f32,

    #[derivative(Default(value = "Dimension::Undefined"))]
    pub flex_basis: Dimension,

    #[derivative(Default(value = "0.0"))]
    pub flex_grow: f32,

    #[derivative(Default(value = "1.0"))]
    pub flex_shrink: f32,

    pub flex_direction: FlexDirection,

    pub align_items: AlignItems,
    pub justify_content: JustifyContent,

    pub padding: Padding,
    pub margin: Margin,

    #[derivative(Default(value = "Dimension::Auto"))]
    pub width: Dimension,
    #[derivative(Default(value = "Dimension::Auto"))]
    pub height: Dimension,

    #[derivative(Default(value = "Dimension::Auto"))]
    pub min_height: Dimension,
    #[derivative(Default(value = "Dimension::Auto"))]
    pub min_width: Dimension,

    #[derivative(Default(value = "Dimension::Auto"))]
    pub max_height: Dimension,
    #[derivative(Default(value = "Dimension::Auto"))]
    pub max_width: Dimension,

    pub transform_translation_x: f32,

    pub visibility: Visibility,
}
