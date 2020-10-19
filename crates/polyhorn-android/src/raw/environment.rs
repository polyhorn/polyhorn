use polyhorn_android_sys::{Activity, Env};
use std::sync::{Arc, RwLock};

use super::Layouter;

/// Opaque type that wraps the shared layouter.
#[derive(Clone)]
pub struct Environment {
    activity: Activity,
    env: Env<'static>,
    layouter: Arc<RwLock<Layouter>>,
}

impl Environment {
    /// Returns a new environment with the given layouter.
    pub fn new(
        activity: Activity,
        env: Env<'static>,
        layouter: Arc<RwLock<Layouter>>,
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
    pub fn layouter(&mut self) -> &Arc<RwLock<Layouter>> {
        &self.layouter
    }
}
