#[cfg(feature = "impl-stretch")]
pub mod stretch;

#[cfg(feature = "impl-yoga")]
pub mod yoga;

use polyhorn_ui::geometry::Dimension;
use polyhorn_ui::styles::ViewStyle;
use std::hash::Hash;

use crate::{Layout, MeasureFunc, Size};

pub trait Algorithm {
    type Node: Copy + Clone + Hash + Eq;

    fn new() -> Self;

    fn new_node(&mut self, style: ViewStyle, children: &[Self::Node]) -> Self::Node;

    fn new_leaf(&mut self, style: ViewStyle, measure: MeasureFunc) -> Self::Node;

    fn add_child(&mut self, parent: Self::Node, child: Self::Node);

    fn remove_child(&mut self, parent: Self::Node, child: Self::Node);

    fn child_count(&self, parent: Self::Node) -> usize;

    fn remove(&mut self, node: Self::Node);

    fn set_style(&mut self, node: Self::Node, style: ViewStyle);

    fn set_measure(&mut self, node: Self::Node, measure: MeasureFunc);

    fn compute_layout(&mut self, node: Self::Node, size: Size<Dimension<f32>>);

    fn layout(&self, node: Self::Node) -> Layout;
}
