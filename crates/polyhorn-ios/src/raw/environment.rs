use polyhorn_ui::layout::LayoutTree;
use std::sync::{Arc, RwLock};

/// Opaque type that wraps the shared layout tree.
pub struct Environment {
    layout_tree: Arc<RwLock<LayoutTree>>,
}

impl Environment {
    /// Returns a new environment with the given layout tree.
    pub fn new(layout_tree: Arc<RwLock<LayoutTree>>) -> Environment {
        Environment { layout_tree }
    }

    /// Returns a reference to the shared layout tree.
    pub fn layout_tree(&mut self) -> &Arc<RwLock<LayoutTree>> {
        &self.layout_tree
    }
}
