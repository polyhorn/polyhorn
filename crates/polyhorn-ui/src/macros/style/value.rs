use strum_macros::EnumString;

use crate::color::Color;
use crate::font::{FontFamily, FontSize, FontStyle, FontWeight};
use crate::geometry::{ByCorner, ByDirection, ByEdge, Dimension};
use crate::layout::LayoutDirection;
use crate::styles::{
    Align, Border, FlexDirection, Inherited, Justify, ObjectFit, Overflow, TextAlign, Transform,
    Visibility,
};

/// Determines whether this view should be included in calculating the layout of
/// descendant views of the ancestor of this view. Note: unlike the internal
/// representation of positions, this enum is shallow (like CSS) and does not
/// contain the position-dependent subfields.
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumString)]
pub enum PositionType {
    /// If layed out absolutely, this view does not affect the layout of its
    /// ancestor or siblings.
    #[strum(serialize = "absolute")]
    Absolute,

    /// If layed out relatively, this view is included in calculating the layout
    /// of its ancestor and siblings.
    #[strum(serialize = "relative")]
    Relative,
}

/// Represents a typed property value.
#[derive(Debug)]
pub enum PropertyValue {
    /// This is the alignment of items along the main axis of the flexbox. The
    /// default value for this property is `Align::Stretch` which will resize
    /// descendant views along the cross axis to match the relevant dimension of
    /// this view.
    AlignItems(Align),

    /// This is the background color of this view. The default color is
    /// transparent. This property does not affect the layout of this view, its
    /// siblings or its descendants.
    BackgroundColor(Color),

    /// Provides the distance of this view to the bottom edge of its ancestor.
    Bottom(Dimension<f32>),

    /// This is the color that will be used to fill the text outlines. If not
    /// present, the Text component will inherit the text color of its parent.
    /// If the parent does not have a color, the default `Color::canvastext()`
    /// system color will be used. Note that the concrete value of this color
    /// is system-dependent and can vary depending on the user's appearance mode
    /// (i.e. light vs. dark mode).
    Color(Inherited<Color>),

    /// This field determines the direction in which descendant views are layed
    /// out.
    Direction(Inherited<LayoutDirection>),

    /// If present, this property controls the weight of this view in computing
    /// a layout using the flexbox algorithm.
    FlexBasis(Dimension<f32>),

    /// This is the main axis along which the flexbox algorithm operates.
    FlexDirection(FlexDirection),

    /// This property controls the priority of this view when the flexbox can
    /// grow. The default value of this property is 0.0, which means that this
    /// view does not grow if more space is available. If set to any non-zero
    /// positive number, this view will consume (a portion of) the remaining
    /// available space of a flexbox.
    FlexGrow(f32),

    /// This property controls the priority of this view when the flexbox is
    /// shrunk. The default value of this property is 1.0, which means that this
    /// view is shrunk when necessary. If set to 0.0, this view will not be
    /// shrunk.
    FlexShrink(f32),

    /// This is the font family that will be used to render the text outlines.
    /// If not present, the Text component will inherit its font family from its
    /// parent. If the parent does not have a font family, the default
    /// `FontFamily::Generic(GenericFontFamily::SansSerif)` will be used. Note
    /// that the concrete value of this font family is system-dependent and can
    /// vary depending on the user's preferred fonts.
    FontFamily(Inherited<FontFamily<String>>),

    /// This is the font style that will be used to render the text outlines. If
    /// not present, the Text component will inherit its font style from its
    /// parent. If the parent does not have a font style, the default
    /// `FontStyle::Normal` will be used.
    FontStyle(Inherited<FontStyle>),

    /// This is the font weight that will be used to render the text outlines.
    /// If not present, the Text component will inherit its font weight from its
    /// parent. If the parent does not have a font weight, the default
    /// `FontWeight::Normal` (400) will be used.
    FontWeight(Inherited<FontWeight>),

    /// This is the font size that will be used to render the text. If not
    /// present, the Text component will inherit its font size from its parent.
    /// If the parent does not have a font size, the default `FontSize:Medium`
    /// will be used. Note that the concrete value of this font size is
    /// system-dependent and can vary depending on a user's preferred font size.
    FontSize(Inherited<FontSize>),

    /// This is the height of the view.
    Height(Dimension<f32>),

    /// This property controls how the flexbox algorithm justifies content along
    /// the cross axis of the flexbox. The default value for this property is
    /// `Justify::FlexStart` which will stick the content to the start of the
    /// main axis.
    JustifyContent(Justify),

    /// Provides the distance of this view to the left edge of its ancestor.
    Left(Dimension<f32>),

    /// This is the maximum height of this view. This must not be less than the
    /// `height` field if both fields are present and contain absolute values.
    MaxHeight(Dimension<f32>),

    /// This is the maximum width of this view. This must not be less than the
    /// `width` field if both fields are present and contain absolute values.
    MaxWidth(Dimension<f32>),

    /// This is the minimum height of this view. This must not be greater than
    /// the `height` field if both fields are present and contain absolute
    /// values.
    MinHeight(Dimension<f32>),

    /// This is the minimum width of this view. This must not be greater than
    /// the `width` field if both fields are present and contain absolute
    /// values.
    MinWidth(Dimension<f32>),

    /// Controls the method for fitting images that do not match the dimensions
    /// of their container.
    ObjectFit(ObjectFit),

    /// If neither 1.0 nor 0.0, this property contains the weight of this layer
    /// during composition. If 0.0, the layer is not composited at all, and if
    /// 1.0, it is composited over any underlying layers. The default value is
    /// 1.0.
    Opacity(f32),

    /// Controls the way dimensions of views are adjusted when their content
    /// overflows their original boundaries.
    Overflow(Overflow),

    /// This property controls the margin that is used outside this view.
    Margin(ByEdge<Dimension<f32>>),

    /// This property controls the border that is rendered around this view.
    Border(ByEdge<Border>),

    /// If not 0.0, this field controls the corner radius of this view. This
    /// property does not affect the layout of this view, its siblings or its
    /// descendants.
    BorderRadius(ByCorner<ByDirection<Dimension<f32>>>),

    /// This property controls the padding that is used inside this view.
    Padding(ByEdge<Dimension<f32>>),

    /// This field determines whether this view should be included in
    /// calculating the layout of descendant views of the ancestor of this view.
    Position(PositionType),

    /// Provides the distance of this view to the right edge of its ancestor.
    Right(Dimension<f32>),

    /// Controls the alignment of text when it is rendered to a container that
    /// is larger than the rendered text.
    TextAlign(Inherited<TextAlign>),

    /// If present, controls the color that this image is rendered in. Only the
    /// alpha channel of the original image is kept: all other channels are
    /// replaced by the given tint color. If this tint color exists in a
    /// different color space than the original image, the resulting image is
    /// drawn using the color space of the tint color.
    TintColor(Color),

    /// Provides the distance of this view to the top edge of its ancestor.
    Top(Dimension<f32>),

    /// Applies a series of 3D affine transformations to this view.
    Transform([Transform<f32>; 8]),

    /// Controls the visibility of this view. Invisible views are still included
    /// in layout calculations, but are not actually rendered to the screen.
    /// This can be used as a performance optimization. Generally, it is more
    /// efficient to set a view's visibility to hidden than to set its opacity
    /// to zero.
    Visibility(Visibility),

    /// This is the width of the view.
    Width(Dimension<f32>),
}
