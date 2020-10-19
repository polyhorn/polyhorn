use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::algorithm::{Algorithm, Flexbox, Node};
use crate::geometry::{Dimension, Point, Size};
use crate::styles::ViewStyle;

/// Measure function that is called to obtain the intrinsic content size of a
/// leaf node (e.g. images or text).
pub enum MeasureFunc {
    /// Measure function backed by a boxed closure.
    Boxed(Box<dyn Fn(Size<Dimension<f32>>) -> Size<f32>>),
}

/// Computed layout of a node.
pub struct Layout {
    /// The origin of a node.
    pub origin: Point<f32>,

    /// The size of a node.
    pub size: Size<f32>,
}

/// LayoutTree that manages the flexbox algorithm and some additional details,
/// like the roots of the layout tree.
pub struct LayoutTree {
    flexbox: Flexbox,
    parents: HashMap<Node, Node>,
    roots: Vec<Node>,
}

impl LayoutTree {
    /// Returns a new layout tree.
    pub fn new() -> LayoutTree {
        LayoutTree {
            flexbox: Flexbox::new(),
            parents: HashMap::new(),
            roots: vec![],
        }
    }

    /// Returns a reference to the underlying flexbox implementation.
    pub fn flexbox(&self) -> &Flexbox {
        &self.flexbox
    }

    /// Returns a mutable reference to the underlying flexbox implementation.
    pub fn flexbox_mut(&mut self) -> &mut Flexbox {
        &mut self.flexbox
    }

    /// Returns a reference to the roots of the layout tree.
    pub fn roots(&self) -> &[Node] {
        self.roots.as_slice()
    }

    /// Returns a mutable reference to the roots of the layout tree.
    pub fn roots_mut(&mut self) -> &mut Vec<Node> {
        &mut self.roots
    }

    /// Adds a child node to a parent node within the layout tree. It also keeps
    /// track of the parent by itself, so that we don't need to remember which
    /// parent the child belongs to when we want to remove it.
    pub fn add_child(&mut self, parent: Node, child: Node) {
        self.parents.insert(child, parent);
        self.flexbox.add_child(parent, child);
    }

    /// Removes the given node from the layout tree. Note that the layout tree
    /// internally stores a reference to the parent node of every child node, so
    /// we don't have to pass that to this function.
    pub fn remove(&mut self, node: Node) {
        if let Some(parent) = self.parents.remove(&node) {
            self.flexbox.remove_child(parent, node);
        }

        assert_eq!(self.flexbox.child_count(node), 0);

        self.flexbox.remove(node);
    }

    /// Recomputes the layout of all roots of the layout tree.
    pub fn recompute_roots(&mut self) {
        for node in self.roots().to_owned() {
            let size = self.flexbox().layout(node).size;

            self.flexbox_mut().compute_layout(
                node,
                Size {
                    width: Dimension::Points(size.width),
                    height: Dimension::Points(size.height),
                },
            );
        }
    }
}

/// Handle to a node within the layout tree.
#[derive(Clone)]
pub struct LayoutNode {
    layouter: Arc<RwLock<LayoutTree>>,
    node: Node,
}

impl LayoutNode {
    /// Creates a new branch in the layout tree.
    pub fn new(layouter: Arc<RwLock<LayoutTree>>) -> LayoutNode {
        let node = layouter
            .write()
            .unwrap()
            .flexbox_mut()
            .new_node(Default::default(), &[]);

        LayoutNode { layouter, node }
    }

    /// Creates a new leaf in the layout tree.
    pub fn leaf(layouter: Arc<RwLock<LayoutTree>>) -> LayoutNode {
        let node = layouter.write().unwrap().flexbox_mut().new_leaf(
            Default::default(),
            MeasureFunc::Boxed(Box::new(|_| Size {
                width: 0.0,
                height: 0.0,
            })),
        );

        LayoutNode { layouter, node }
    }

    /// Returns a shared reference to the layout tree.
    pub fn layouter(&self) -> &Arc<RwLock<LayoutTree>> {
        &self.layouter
    }

    /// Returns the ID of this node.
    pub fn node(&self) -> Node {
        self.node
    }

    /// Updates the style of this node without recomputing its layout or any of
    /// its ancestors' layouts.
    pub fn set_style(&self, style: ViewStyle) {
        self.layouter
            .write()
            .unwrap()
            .flexbox_mut()
            .set_style(self.node, style)
    }

    /// Recomputes the layout of this node, respecting the given container size.
    pub fn compute(&mut self, size: Option<(f32, f32)>) {
        let size = match size {
            Some((width, height)) => Size {
                width: Dimension::Points(width),
                height: Dimension::Points(height),
            },
            None => Size {
                width: Dimension::Undefined,
                height: Dimension::Undefined,
            },
        };

        self.layouter
            .write()
            .unwrap()
            .flexbox_mut()
            .compute_layout(self.node, size);
    }

    /// Returns the current computed layout.
    pub fn current(&self) -> Layout {
        self.layouter.read().unwrap().flexbox().layout(self.node)
    }
}
