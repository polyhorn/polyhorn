#[cfg(feature = "layout-stretch")]
pub mod stretch;

/// Flexbox implementation powered by Yoga (the default).
#[cfg(feature = "layout-yoga")]
pub mod yoga;

#[cfg(feature = "layout-stretch")]
pub use self::stretch::*;

#[cfg(feature = "layout-yoga")]
pub use self::yoga::*;

use std::hash::Hash;

use crate::geometry::Dimension;
use crate::styles::ViewStyle;

use super::{Layout, MeasureFunc, Size};

/// Abstract interface of a flexbox algorithm.
pub trait Algorithm {
    /// Represents the type of a node within this flexbox.
    type Node: Copy + Clone + Hash + Eq;

    /// Create a new instance of the flexbox (i.e. root).
    fn new() -> Self;

    /// Create a new node with the given child nodes.
    fn new_node(&mut self, style: ViewStyle, children: &[Self::Node]) -> Self::Node;

    /// Create a new leaf with an intrinsic content size that is determined by
    /// the given measure function.
    fn new_leaf(&mut self, style: ViewStyle, measure: MeasureFunc) -> Self::Node;

    /// Add a child node to a parent node.
    fn add_child(&mut self, parent: Self::Node, child: Self::Node);

    /// Remove a child node from a parent node.
    fn remove_child(&mut self, parent: Self::Node, child: Self::Node);

    /// Returns the number of child nodes of a specific parent node.
    fn child_count(&self, parent: Self::Node) -> usize;

    /// Removes the given node from this flexbox.
    fn remove(&mut self, node: Self::Node);

    /// Updates the style of a specific node within this flexbox.
    fn set_style(&mut self, node: Self::Node, style: ViewStyle);

    /// Updates the measure function of a specific node within this flexbox.
    fn set_measure(&mut self, node: Self::Node, measure: MeasureFunc);

    /// Computes the layout of a node within this flexbox within the given
    /// size.
    fn compute_layout(&mut self, node: Self::Node, size: Size<Dimension<f32>>);

    /// Returns the computed layout of a node within this flexbox.
    fn layout(&self, node: Self::Node) -> Layout;
}
