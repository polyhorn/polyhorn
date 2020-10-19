use polyhorn_android_sys::{Activity, Env};
use polyhorn_ui::layout::LayoutTree;
use std::sync::{Arc, RwLock};

/// Opaque type that wraps the shared layouter.
#[derive(Clone)]
pub struct Environment {
    activity: Activity,
    env: Env<'static>,
    layouter: Arc<RwLock<LayoutTree>>,
}

impl Environment {
    /// Returns a new environment with the given layouter.
    pub fn new(
        activity: Activity,
        env: Env<'static>,
        layouter: Arc<RwLock<LayoutTree>>,
    ) -> Environment {
        Environment {
            activity,
            env,
            layouter,
        }
    }

    pub fn activity(&self) -> &Activity {
        &self.activity
    }

    pub fn env(&self) -> &Env {
        &self.env
    }

    /// Returns a reference to the shared layouter.
    pub fn layouter(&mut self) -> &Arc<RwLock<LayoutTree>> {
        &self.layouter
    }
}
