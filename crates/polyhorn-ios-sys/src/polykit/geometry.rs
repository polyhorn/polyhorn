use crate::coregraphics::CGFloat;

/// An abstract type representing a dimensional unit of measure.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PLYDimension {
    /// The unit of this dimension.
    pub kind: PLYDimensionKind,

    /// The value of this dimension.
    pub value: CGFloat,
}

/// Represents the unit of a dimension.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum PLYDimensionKind {
    /// Absolute unit of measure for display points.
    Pixels,

    /// Relative unit of measure.
    Percentage,
}

/// A structure that contains a point in a two-dimensional coordinate system.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PLYPoint {
    /// The x-coordinate of the point.
    pub x: PLYDimension,

    /// The y-coordinate of the point.
    pub y: PLYDimension,
}

impl PLYPoint {
    /// Returns a point with the specified coordinates.
    pub fn new(x: PLYDimension, y: PLYDimension) -> PLYPoint {
        PLYPoint { x, y }
    }
}

/// Abstract type that contains a value of the given type for each horizontal
/// edge of a rectangle.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PLYLayoutAxisX<T> {
    /// Whether the this axis is layout direction independent or not.
    pub independent: bool,

    /// Value for the left edge or leading edge if independent is false.
    pub start: T,

    /// Value for the right edge or trailing edge if independent is false.
    pub end: T,
}

/// An abstract type that contains a value of the given type for each vertical
/// edge of a rectangle.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PLYLayoutAxisY<T> {
    /// Value for the top edge.
    pub top: T,

    /// Value for the bottom edge.
    pub bottom: T,
}

/// An abstract type that contains a value of the given type for each edge of a
/// rectangle.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PLYByEdge<T> {
    /// Values for the horizontal edges.
    pub horizontal: PLYLayoutAxisX<T>,

    /// Values for the vertical edges.
    pub vertical: PLYLayoutAxisY<T>,
}

/// The radius to use when drawing rounded corners for a view's background.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PLYCornerRadii {
    /// The radius to use when drawing the rounded top left corner for a view's
    /// background.
    pub top_left: PLYPoint,

    /// The radius to use when drawing the rounded top right corner for a view's
    /// background.
    pub top_right: PLYPoint,

    /// The radius to use when drawing the rounded bottom left corner for a
    /// view's background.
    pub bottom_right: PLYPoint,

    /// The radius to use when drawing the rounded bottom right corner for a
    /// view's background.
    pub bottom_left: PLYPoint,
}

/// The inset distances for views.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PLYEdgeInsets {
    /// The top edge inset value.
    pub top: CGFloat,

    /// The left edge inset value.
    pub left: CGFloat,

    /// The bottom edge inset value.
    pub bottom: CGFloat,

    /// The right edge inset value.
    pub right: CGFloat,
}
