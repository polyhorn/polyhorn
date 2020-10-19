use polyhorn_android_sys::{Activity, Env};
use polyhorn_ui::layout::LayoutTree;
use std::sync::{Arc, RwLock};

/// Opaque type that wraps the shared layout tree.
#[derive(Clone)]
pub struct Environment {
    activity: Activity,
    env: Env<'static>,
    layout_tree: Arc<RwLock<LayoutTree>>,
}

impl Environment {
    /// Returns a new environment with the given layout tree.
    pub fn new(
        activity: Activity,
        env: Env<'static>,
        layout_tree: Arc<RwLock<LayoutTree>>,
    ) -> Environment {
        Environment {
            activity,
            env,
            layout_tree,
        }
    }

    pub fn activity(&self) -> &Activity {
        &self.activity
    }

    pub fn env(&self) -> &Env {
        &self.env
    }

    /// Returns a reference to the shared layout tree.
    pub fn layout_tree(&mut self) -> &Arc<RwLock<LayoutTree>> {
        &self.layout_tree
    }
}
