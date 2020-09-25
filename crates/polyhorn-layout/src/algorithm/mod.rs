#[cfg(feature = "impl-stretch")]
pub mod stretch;

#[cfg(feature = "impl-yoga")]
pub mod yoga;

use crate::{Layout, MeasureFunc, Size, Style};
use polyhorn_style::Dimension;

use std::hash::Hash;

pub trait Algorithm {
    type Node: Copy + Clone + Hash + Eq;

    fn new() -> Self;

    fn new_node(&mut self, style: Style, children: &[Self::Node]) -> Self::Node;

    fn new_leaf(&mut self, style: Style, measure: MeasureFunc) -> Self::Node;

    fn add_child(&mut self, parent: Self::Node, child: Self::Node);

    fn remove_child(&mut self, parent: Self::Node, child: Self::Node);

    fn child_count(&self, parent: Self::Node) -> usize;

    fn remove(&mut self, node: Self::Node);

    fn set_style(&mut self, node: Self::Node, style: Style);

    fn set_measure(&mut self, node: Self::Node, measure: MeasureFunc);

    fn compute_layout(&mut self, node: Self::Node, size: Size<Dimension>);

    fn layout(&self, node: Self::Node) -> Layout;
}
