//! Primitives to work with layout and opaque geometry.

use num_traits::Float;
use std::borrow::Borrow;
use strum_macros::EnumString;

use crate::geometry::Size;

/// Implementations of the flexbox algorithm.
pub mod algorithm;
mod tree;

pub use algorithm::Algorithm;
pub use tree::{Layout, LayoutNode, LayoutTree, MeasureFunc};

/// Opaque coordinate within a reference system.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct LayoutAnchor<T>
where
    T: Float,
{
    x: T,
    y: T,
}

impl<T> LayoutAnchor<T>
where
    T: Float,
{
    /// Returns a new anchor with the given absolute coordinates (i.e. relative
    /// to the origin of the reference system).
    pub fn new(x: T, y: T) -> LayoutAnchor<T> {
        LayoutAnchor { x, y }
    }

    /// Returns the distance between this anchor and the given anchor.
    pub fn distance(self, other: Self) -> LayoutDistance<T> {
        LayoutDistance {
            dx: other.borrow().x - self.x,
            dy: other.borrow().y - self.y,
        }
    }
}

/// Represents the layout direction of a language (i.e. left-to-right vs.
/// right-to-left).
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumString)]
pub enum LayoutDirection {
    /// Left to right. This is the most commonly used direction in most
    /// languages.
    #[strum(serialize = "ltr")]
    LTR,

    /// Right to left. This direction is used in Hebrew and Arabic, for example.
    #[strum(serialize = "rtl")]
    RTL,
}

/// Represents a potentially direction dependent horizontal layout axis.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LayoutAxisX<T> {
    /// A horizontal axis that depends on the layout direction.
    DirectionDependent {
        /// Contains the layout direction dependent leading value for this
        /// horizontal axis.
        leading: T,

        /// Contains the layout direction dependent leading value for this
        /// horizontal axis.
        trailing: T,
    },

    /// A horizontal axis that is independent of the layout direction.
    DirectionIndependent {
        /// Contains the layout direction independent left value for this
        /// horizontal axis.
        left: T,

        /// Contains the layout direction independent right value for this
        /// horizontal axis.
        right: T,
    },
}

impl<T> LayoutAxisX<T> {
    /// Returns a layout direction dependent horizontal axis with the given
    /// leading and trailing values.
    pub fn dependent(leading: T, trailing: T) -> LayoutAxisX<T> {
        LayoutAxisX::DirectionDependent { leading, trailing }
    }

    /// Returns a layout direction independent horizontal axis with the given
    /// left and right values.
    pub fn independent(left: T, right: T) -> LayoutAxisX<T> {
        LayoutAxisX::DirectionIndependent { left, right }
    }

    /// Resolves the left value of this horizontal axis using the given layout
    /// direction.
    pub fn left(&self, direction: LayoutDirection) -> &T {
        match self {
            LayoutAxisX::DirectionDependent { leading, trailing } => match direction {
                LayoutDirection::LTR => leading,
                LayoutDirection::RTL => trailing,
            },
            LayoutAxisX::DirectionIndependent { left, .. } => left,
        }
    }

    /// Resolves the right value of this horizontal axis using the given layout
    /// direction.
    pub fn right(&self, direction: LayoutDirection) -> &T {
        match self {
            LayoutAxisX::DirectionDependent { leading, trailing } => match direction {
                LayoutDirection::LTR => trailing,
                LayoutDirection::RTL => leading,
            },
            LayoutAxisX::DirectionIndependent { right, .. } => right,
        }
    }
}

impl<T> Default for LayoutAxisX<T>
where
    T: Default,
{
    fn default() -> Self {
        LayoutAxisX::DirectionIndependent {
            left: T::default(),
            right: T::default(),
        }
    }
}

/// Represents a direction independent vertical layout axis.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct LayoutAxisY<T> {
    /// Contains the horizontal axis for the top edge.
    pub top: T,

    /// Contains the horizontal axis for the bottom edge.
    pub bottom: T,
}

/// Opaque rectangle defined by coordinates within a reference system.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LayoutGuide<T>
where
    T: Float,
{
    origin: LayoutAnchor<T>,
    size: Size<T>,
    direction: LayoutDirection,
}

impl<T> LayoutGuide<T>
where
    T: Float,
{
    /// Returns a new layout guide with the given opaque origin coordinate and
    /// the given concrete size and layout direction.
    pub fn new(
        origin: LayoutAnchor<T>,
        size: Size<T>,
        direction: LayoutDirection,
    ) -> LayoutGuide<T> {
        LayoutGuide {
            origin,
            size,
            direction,
        }
    }

    /// Returns a layout anchor for the top left corner.
    pub fn top_left(&self) -> LayoutAnchor<T> {
        self.origin
    }

    /// Returns a layout anchor for the top right corner.
    pub fn top_right(&self) -> LayoutAnchor<T> {
        let mut anchor = self.origin;
        anchor.x = anchor.x + self.size.width;
        anchor
    }

    /// Returns a layout anchor for the top leading corner (which is either the
    /// top left corner or the top right corner depending on the layout
    /// direction).
    pub fn top_leading(&self) -> LayoutAnchor<T> {
        match self.direction {
            LayoutDirection::LTR => self.top_left(),
            LayoutDirection::RTL => self.top_right(),
        }
    }

    /// Returns a layout anchor for the top trailing corner (which is either the
    /// top right corner or the top left corner depending on the layout
    /// direction).
    pub fn top_trailing(&self) -> LayoutAnchor<T> {
        match self.direction {
            LayoutDirection::LTR => self.top_right(),
            LayoutDirection::RTL => self.top_left(),
        }
    }

    /// Returns a layout anchor for the bottom left corner.
    pub fn bottom_left(&self) -> LayoutAnchor<T> {
        let mut anchor = self.origin;
        anchor.y = anchor.y + self.size.height;
        anchor
    }

    /// Returns a layout anchor for the bottom right corner.
    pub fn bottom_right(&self) -> LayoutAnchor<T> {
        let mut anchor = self.origin;
        anchor.x = anchor.x + self.size.width;
        anchor.y = anchor.y + self.size.height;
        anchor
    }

    /// Returns a layout anchor for the bottom leading corner (which is either
    /// the bottom left corner or the bottom right corner depending on the
    /// layout direction).
    pub fn bottom_leading(&self) -> LayoutAnchor<T> {
        match self.direction {
            LayoutDirection::LTR => self.bottom_left(),
            LayoutDirection::RTL => self.bottom_right(),
        }
    }

    /// Returns a layout anchor for the bottom trailing corner (which is either
    /// the bottom right corner or the bottom left corner depending on the
    /// layout direction).
    pub fn bottom_trailing(&self) -> LayoutAnchor<T> {
        match self.direction {
            LayoutDirection::LTR => self.bottom_right(),
            LayoutDirection::RTL => self.bottom_left(),
        }
    }
}

/// Distance between opaque coordinates within a reference system.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct LayoutDistance<T> {
    dx: T,
    dy: T,
}

impl<T> LayoutDistance<T>
where
    T: Float,
{
    /// Returns the L1 norm of this distance vector.
    pub fn l1_norm(&self) -> T
    where
        T: Float,
    {
        self.dx.abs() + self.dy.abs()
    }

    /// Returns the L2 norm of this distance vector.
    pub fn l2_norm(&self) -> T {
        self.dx * self.dx + self.dy * self.dy
    }
}

#[cfg(test)]
mod tests {
    use super::LayoutAnchor;

    #[test]
    fn test_l1_norm() {
        let a = LayoutAnchor::new(4.0, -42.0);
        let b = LayoutAnchor::new(-2.0, -8.0);

        assert_eq!(a.distance(b).l1_norm(), 40.0);
        assert_eq!(b.distance(a).l1_norm(), 40.0);
    }

    #[test]
    fn test_l2_norm() {
        let a = LayoutAnchor::new(4.0, -42.0);
        let b = LayoutAnchor::new(-2.0, -8.0);

        assert_eq!(a.distance(b).l2_norm(), 1192.0);
        assert_eq!(b.distance(a).l2_norm(), 1192.0);
    }
}
