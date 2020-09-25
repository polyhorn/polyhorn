use polyhorn_layout::algorithm::yoga::{Flexbox, Node};
use polyhorn_layout::algorithm::Algorithm;
use polyhorn_layout::{MeasureFunc, Size, Style};
use polyhorn_style::{Dimension, IntoDimension};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct Layouter {
    flexbox: Flexbox,
    parents: HashMap<Node, Node>,
    roots: Vec<Node>,
}

impl Layouter {
    pub fn new() -> Layouter {
        Layouter {
            flexbox: Flexbox::new(),
            parents: HashMap::new(),
            roots: vec![],
        }
    }

    pub fn flexbox(&self) -> &Flexbox {
        &self.flexbox
    }

    pub fn flexbox_mut(&mut self) -> &mut Flexbox {
        &mut self.flexbox
    }

    pub fn roots(&self) -> &[Node] {
        self.roots.as_slice()
    }

    pub fn roots_mut(&mut self) -> &mut Vec<Node> {
        &mut self.roots
    }

    pub fn add_child(&mut self, parent: Node, child: Node) {
        self.parents.insert(child, parent);
        self.flexbox.add_child(parent, child);
    }

    pub fn remove(&mut self, node: Node) {
        if let Some(parent) = self.parents.remove(&node) {
            self.flexbox.remove_child(parent, node);
        }

        assert_eq!(self.flexbox.child_count(node), 0);

        self.flexbox.remove(node);
    }

    pub fn recompute_roots(&mut self) {
        for node in self.roots().to_owned() {
            let size = self.flexbox().layout(node).size;

            self.flexbox_mut().compute_layout(
                node,
                Size {
                    width: Dimension::Pixels(size.width),
                    height: Dimension::Pixels(size.height),
                },
            );
        }
    }
}

unsafe impl Send for Layouter {}
unsafe impl Sync for Layouter {}

#[derive(Clone)]
pub struct Layout {
    layouter: Arc<RwLock<Layouter>>,
    node: Node,
}

impl Layout {
    pub fn new(layouter: Arc<RwLock<Layouter>>) -> Layout {
        let node = layouter
            .write()
            .unwrap()
            .flexbox_mut()
            .new_node(Default::default(), &[]);

        Layout { layouter, node }
    }

    pub fn leaf(layouter: Arc<RwLock<Layouter>>) -> Layout {
        let node = layouter.write().unwrap().flexbox_mut().new_leaf(
            Default::default(),
            MeasureFunc::Boxed(Box::new(|_| Size {
                width: 0.0,
                height: 0.0,
            })),
        );

        Layout { layouter, node }
    }

    pub fn layouter(&self) -> &Arc<RwLock<Layouter>> {
        &self.layouter
    }

    pub fn node(&self) -> Node {
        self.node
    }

    pub fn set_style(&self, style: Style) {
        self.layouter
            .write()
            .unwrap()
            .flexbox_mut()
            .set_style(self.node, style)
    }

    pub fn compute(&mut self, size: Option<(f32, f32)>) {
        let size = match size {
            Some((width, height)) => Size {
                width: width.px(),
                height: height.px(),
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

    pub fn current(&self) -> polyhorn_layout::Layout {
        self.layouter.read().unwrap().flexbox().layout(self.node)
    }
}
