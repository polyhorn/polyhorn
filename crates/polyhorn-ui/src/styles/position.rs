use crate::geometry::{ByEdge, Dimension};

/// Controls the absolute positioning of a view.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Absolute {
    /// Provides the distance of this view to each of the edges of its ancestor.
    pub distances: ByEdge<Dimension<f32>>,
}

/// Controls the relative positioning of a view.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Relative {
    /// If present, this property controls the weight of this view in computing
    /// a layout using the flexbox algorithm.
    pub flex_basis: Dimension<f32>,

    /// This property controls the priority of this view when the flexbox can
    /// grow. The default value of this property is 0.0, which means that this
    /// view does not grow if more space is available. If set to any non-zero
    /// positive number, this view will consume (a portion of) the remaining
    /// available space of a flexbox.
    pub flex_grow: f32,

    /// This property controls the priority of this view when the flexbox is
    /// shrunk. The default value of this property is 1.0, which means that this
    /// view is shrunk when necessary. If set to 0.0, this view will not be
    /// shrunk.
    pub flex_shrink: f32,
}

impl Default for Relative {
    fn default() -> Self {
        Relative {
            flex_basis: Dimension::Undefined,
            flex_grow: 0.0,
            flex_shrink: 1.0,
        }
    }
}

/// Determines whether a view affects the layout of its ancestor and siblings.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Position {
    /// If layed out absolutely, this view does not affect the layout of its
    /// ancestor or siblings.
    Absolute(Absolute),

    /// If layed out relatively, this view is included in calculating the layout
    /// of its ancestor and siblings.
    Relative(Relative),
}
