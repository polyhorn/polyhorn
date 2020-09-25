mod convert;

pub use convert::IntoStretch;

use super::Algorithm;
use crate::{Layout, MeasureFunc, Point, Size, Style};
use polyhorn_style::Dimension;

pub struct Flexbox(stretch::Stretch);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Node(stretch::node::Node);

impl Algorithm for Flexbox {
    type Node = Node;

    fn new() -> Flexbox {
        Flexbox(stretch::Stretch::new())
    }

    fn new_node(&mut self, style: Style, children: &[Self::Node]) -> Self::Node {
        Node(
            self.0
                .new_node(
                    style.into_stretch(),
                    &children.into_iter().map(|node| node.0).collect::<Vec<_>>(),
                )
                .unwrap(),
        )
    }

    fn new_leaf(&mut self, style: Style, measure: MeasureFunc) -> Self::Node {
        Node(
            self.0
                .new_leaf(style.into_stretch(), measure.into_stretch())
                .unwrap(),
        )
    }

    fn add_child(&mut self, parent: Self::Node, child: Self::Node) {
        self.0.add_child(parent.0, child.0).unwrap();
    }

    fn remove_child(&mut self, parent: Self::Node, child: Self::Node) {
        self.0.remove_child(parent.0, child.0).unwrap();
    }

    fn child_count(&self, parent: Self::Node) -> usize {
        self.0.child_count(parent.0).unwrap()
    }

    fn remove(&mut self, node: Self::Node) {
        self.0.remove(node.0);
    }

    fn set_style(&mut self, node: Self::Node, style: Style) {
        self.0.set_style(node.0, style.into_stretch()).unwrap();
    }

    fn set_measure(&mut self, node: Self::Node, measure: MeasureFunc) {
        self.0
            .set_measure(node.0, Some(measure.into_stretch()))
            .unwrap();
    }

    fn compute_layout(&mut self, node: Self::Node, size: Size<Dimension>) {
        self.0
            .compute_layout(
                node.0,
                stretch::geometry::Size {
                    width: match size.width {
                        Dimension::Pixels(pixels) => stretch::number::Number::Defined(pixels),
                        _ => stretch::number::Number::Undefined,
                    },
                    height: match size.height {
                        Dimension::Pixels(pixels) => stretch::number::Number::Defined(pixels),
                        _ => stretch::number::Number::Undefined,
                    },
                },
            )
            .unwrap();
    }

    fn layout(&self, node: Self::Node) -> Layout {
        let layout = self.0.layout(node.0).unwrap();

        Layout {
            origin: Point {
                x: layout.location.x,
                y: layout.location.y,
            },
            size: Size {
                width: layout.size.width,
                height: layout.size.height,
            },
        }
    }
}
