use std::fmt::Debug;
use strum_macros::EnumString;

use super::{Align, FlexDirection, Inherited, Justify, Position, Transform};
use crate::color::Color;
use crate::geometry::{ByCorner, ByDirection, ByEdge, Dimension, Size};
use crate::layout::LayoutDirection;

/// Controls the style that is used to draw a border.
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumString)]
pub enum BorderStyle {
    /// Draws a solid line.
    #[strum(serialize = "solid")]
    Solid,

    /// Draws a dashed line. The gap and dash lengths, as well as the starting
    /// position of the pattern are currently platform dependent and cannot be
    /// customized.
    #[strum(serialize = "dashed")]
    Dashed,

    /// Draws a dotted line. The starting position of the pattern is currently
    /// platform dependent and cannot be customized.
    #[strum(serialize = "dotted")]
    Dotted,
}

/// Controls the appearance of a border shown around the dimensions of a view.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Border {
    /// Controls the thickness of a border rendered around a view. If the
    /// dimension resolves to either undefined or auto, no border will be shown.
    /// If the dimension is a percentage, it will be resolved relative to the
    /// width of the view's bounding box. Note: even if this is a vertical
    /// border, it will still be resolved relative to the width.
    pub width: Dimension<f32>,

    /// This is the style that is used to draw the border.
    pub style: BorderStyle,

    /// This is the color that is used to draw the border. If the color is
    /// translucent (i.e. the alpha channel is not 1.0), the border color will
    /// be composited over the background color of the view (if present) using
    /// gamma-correct color blending. If the view itself is translucent or
    /// transparent, the platform's own display compositor might use
    /// non-gamma-correct color blending when compositing this view on top of
    /// any underlying views.
    pub color: Color,
}

impl Default for Border {
    fn default() -> Self {
        Border {
            width: Dimension::Undefined,
            style: BorderStyle::Solid,
            color: Color::transparent(),
        }
    }
}

/// Controls the way dimensions of views are adjusted when their content
/// overflows their original boundaries.
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumString)]
pub enum Overflow {
    /// If overflow is visible, views are adjusted to accommodate the larger
    /// content size.
    #[strum(serialize = "visible")]
    Visible,

    /// If overflow is hidden, views are not adjusted.
    #[strum(serialize = "hidden")]
    Hidden,

    /// If overflow is scroll, the view itself is not adjusted, but its
    /// dimensions are increased internally in the layout algorithm to account
    /// for the fact that it can be scrolled.
    #[strum(serialize = "scroll")]
    Scroll,
}

impl Default for Overflow {
    fn default() -> Self {
        Overflow::Visible
    }
}

/// Controls the visibility of a view.
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumString)]
pub enum Visibility {
    /// If visible, the view is both included in layout calculations and
    /// rendered to the screen, even if its opacity is zero.
    #[strum(serialize = "visible")]
    Visible,

    /// If hidden, the view is included in layout calculations, but is not
    /// rendered to the screen, regardless of its opacity.
    #[strum(serialize = "hidden")]
    Hidden,
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Visible
    }
}

/// Controls the appearance of a View.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ViewStyle {
    /// This field determines whether this view should be included in
    /// calculating the layout of descendant views of the ancestor of this view.
    pub position: Position,

    /// This field determines the direction in which descendant views are layed
    /// out.
    pub direction: Inherited<LayoutDirection>,

    /// This is the size of this view.
    pub size: Size<Dimension<f32>>,

    /// This is the minimum size of this view. This must not be greater than the
    /// `size` field if both fields are present and contain absolute values for
    /// one of both dimensions.
    pub min_size: Size<Dimension<f32>>,

    /// This is the maximum size of this view. This must not be less than the
    /// `size` field if both fields are present and contain absolute values for
    /// one of both dimensions.
    pub max_size: Size<Dimension<f32>>,

    /// This is the main axis along which the flexbox algorithm operates.
    pub flex_direction: FlexDirection,

    /// This is the alignment of items along the main axis of the flexbox. The
    /// default value for this property is `Align::Stretch` which will resize
    /// descendant views along the cross axis to match the relevant dimension of
    /// this view.
    pub align_items: Align,

    /// This property controls how the flexbox algorithm justifies content along
    /// the cross axis of the flexbox. The default value for this property is
    /// `Justify::FlexStart` which will stick the content to the start of the
    /// main axis.
    pub justify_content: Justify,

    /// This property controls the margin that is used outside this view.
    pub margin: ByEdge<Dimension<f32>>,

    /// This property controls the border that is rendered around this view.
    pub border: ByEdge<Border>,

    /// This property controls the padding that is used inside this view.
    pub padding: ByEdge<Dimension<f32>>,

    /// This is the background color of this view. The default color is
    /// transparent. This property does not affect the layout of this view, its
    /// siblings or its descendants.
    pub background_color: Color,

    /// If not 0.0, this field controls the corner radius of this view. This
    /// property does not affect the layout of this view, its siblings or its
    /// descendants.
    pub border_radius: ByCorner<ByDirection<Dimension<f32>>>,

    /// If neither 1.0 nor 0.0, this property contains the weight of this layer
    /// during composition. If 0.0, the layer is not composited at all, and if
    /// 1.0, it is composited over any underlying layers. The default value is
    /// 1.0.
    pub opacity: f32,

    /// Applies a transformation to the view during compositing. This property
    /// does not affect the layout of this view, its siblings or descendants.
    pub transform: [Transform<f32>; 8],

    /// Controls the way dimensions of views are adjusted when their content
    /// overflows their original boundaries.
    pub overflow: Overflow,

    /// Controls the visibility of this view. Invisible views are still included
    /// in layout calculations, but are not actually rendered to the screen.
    /// This can be used as a performance optimization. Generally, it is more
    /// efficient to set a view's visibility to hidden than to set its opacity
    /// to zero.
    pub visibility: Visibility,
}

impl Default for ViewStyle {
    fn default() -> Self {
        ViewStyle {
            position: Position::Relative(Default::default()),
            direction: Inherited::Inherited,
            min_size: Size::new(Dimension::Auto, Dimension::Auto),
            max_size: Size::new(Dimension::Auto, Dimension::Auto),
            flex_direction: FlexDirection::Column,
            align_items: Align::Stretch,
            justify_content: Justify::FlexStart,
            size: Size::new(Dimension::Auto, Dimension::Auto),
            background_color: Color::transparent(),
            margin: Default::default(),
            border: Default::default(),
            border_radius: Default::default(),
            padding: Default::default(),
            opacity: 1.0,
            transform: Default::default(),
            overflow: Overflow::Visible,
            visibility: Visibility::Visible,
        }
    }
}
