use super::CGFloat;

/// A structure that contains a point in a two-dimensional coordinate system.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CGPoint {
    /// The x-coordinate of this point.
    pub x: CGFloat,

    /// The y-coordinate of this point.
    pub y: CGFloat,
}

impl CGPoint {
    /// Creates a point with coordinates specified as `CGFloat` values.
    pub fn new(x: CGFloat, y: CGFloat) -> CGPoint {
        CGPoint { x, y }
    }
}

/// A structure that contains width and height values.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CGSize {
    /// A width value.
    pub width: CGFloat,

    /// A height value.
    pub height: CGFloat,
}

impl CGSize {
    /// Creates a size with dimensions specified as `CGFloat` values.
    pub fn new(width: CGFloat, height: CGFloat) -> CGSize {
        CGSize { width, height }
    }
}

/// A structure that contains the location and dimensions of a rectangle.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CGRect {
    /// A point that specifies the coordinates of the rectangle's origin.
    pub origin: CGPoint,

    /// A point that specifies the height and width of the rectangle.
    pub size: CGSize,
}

impl CGRect {
    /// Creates a rectangle with coordinates and dimensions specified as
    /// `CGFloat` values.
    pub fn new(x: CGFloat, y: CGFloat, width: CGFloat, height: CGFloat) -> CGRect {
        CGRect {
            origin: CGPoint::new(x, y),
            size: CGSize::new(width, height),
        }
    }
}
