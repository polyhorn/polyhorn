//! Primitives to work with concrete geometry.

use std::ops::{Deref, DerefMut};
use strum_macros::EnumString;

use crate::layout::{LayoutAxisX, LayoutAxisY};

/// Simple wrapper around the coordinates of a 2D object.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Point<T> {
    /// This is the horizontal coordinate of this point.
    pub x: T,

    /// This is the vertical coordinate of this point.
    pub y: T,
}

impl<T> Point<T> {
    /// Returns a new point with the given coordinates.
    pub const fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

/// Simple wrapper around the horizontal and vertical dimensions of a 2D object.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Size<T> {
    /// This is the horizontal component of a size.
    pub width: T,

    /// This is the vertical component of a size.
    pub height: T,
}

impl<T> Size<T> {
    /// Returns a new size with the given width and height.
    pub const fn new(width: T, height: T) -> Size<T> {
        Size { width, height }
    }
}

/// Represents an absolute or relative dimension.
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumString)]
pub enum Dimension<T> {
    /// This is the default value for a dimension and resembles
    /// `Dimension:::Points(0.0)` and `Dimension::Percentage(0.0)`.
    #[strum(serialize = "undefined")]
    Undefined,

    /// Depending on the property that this dimension is used to, this value may
    /// have a special meaning. Otherwise, it's similar to
    /// `Dimension::Undefined`.
    #[strum(serialize = "auto")]
    Auto,

    /// This is a dimension expressed in absolute units, where each unit
    /// represents a single pixel.
    #[strum(disabled)]
    Points(T),

    /// This is a dimension expressed in relative units, where 1.0 represents
    /// 100%.
    #[strum(disabled)]
    Percentage(T),
}

impl<T> Default for Dimension<T> {
    fn default() -> Self {
        Dimension::Undefined
    }
}

/// This is a wrapper that contains a value of the given type for each corner of
/// a rectangle.
///
/// ```rust
/// use polyhorn_ui::geometry::ByCorner;
/// use polyhorn_ui::layout::{LayoutAxisX, LayoutDirection};
///
/// let mut by_corner = ByCorner::<f32>::default();
/// by_corner.top = LayoutAxisX::dependent(10.0, 30.0);
///
/// assert_eq!(by_corner.top.left(LayoutDirection::LTR), &10.0);
/// assert_eq!(by_corner.top.left(LayoutDirection::RTL), &30.0);
/// ```
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct ByCorner<T> {
    /// This is a nested field that contains a potentially layout direction
    /// dependent horizontal axis in a layout direction independent vertical
    /// axis.
    pub all: LayoutAxisY<LayoutAxisX<T>>,
}

impl<T> ByCorner<T> {
    /// Applies the given operation to every element of this structure and
    /// returns the result. The operation does not necessarily have to return
    /// a value of the same type.
    pub fn map<F, O>(self, mut op: F) -> ByCorner<O>
    where
        F: FnMut(T) -> O,
    {
        ByCorner {
            all: LayoutAxisY {
                top: match self.all.top {
                    LayoutAxisX::DirectionDependent { leading, trailing } => {
                        LayoutAxisX::DirectionDependent {
                            leading: op(leading),
                            trailing: op(trailing),
                        }
                    }
                    LayoutAxisX::DirectionIndependent { left, right } => {
                        LayoutAxisX::DirectionIndependent {
                            left: op(left),
                            right: op(right),
                        }
                    }
                },
                bottom: match self.all.bottom {
                    LayoutAxisX::DirectionDependent { leading, trailing } => {
                        LayoutAxisX::DirectionDependent {
                            leading: op(leading),
                            trailing: op(trailing),
                        }
                    }
                    LayoutAxisX::DirectionIndependent { left, right } => {
                        LayoutAxisX::DirectionIndependent {
                            left: op(left),
                            right: op(right),
                        }
                    }
                },
            },
        }
    }
}

impl<T> Deref for ByCorner<T> {
    type Target = LayoutAxisY<LayoutAxisX<T>>;

    fn deref(&self) -> &Self::Target {
        &self.all
    }
}

impl<T> DerefMut for ByCorner<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.all
    }
}

/// This is a wrapper that contains a value of the given type for each direction
/// (i.e. horizontal and vertical).
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct ByDirection<T> {
    /// This field contains a value of the given type for the horizontal
    /// dimension.
    pub horizontal: T,

    /// This field contains a value of the given type for the vertical
    /// dimension.
    pub vertical: T,
}

impl<T> ByDirection<T>
where
    T: Copy,
{
    /// Returns a new `ByDirection` with the given value of both horizontal and
    /// vertical directions.
    pub fn with_both(value: T) -> ByDirection<T> {
        ByDirection {
            horizontal: value,
            vertical: value,
        }
    }
}

/// This is a wrapper that contains a value of the given type for each edge of a
/// rectangle.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct ByEdge<T> {
    /// This field contains the horizontal edges (i.e. either left and right, or
    /// leading and trailing edges).
    pub horizontal: LayoutAxisX<T>,

    /// This field contains the vertical edges (i.e. top and bottom).
    pub vertical: LayoutAxisY<T>,
}
