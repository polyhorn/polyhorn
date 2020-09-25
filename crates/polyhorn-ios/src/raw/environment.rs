use super::Layouter;
use std::sync::{Arc, RwLock};

/// Opaque type that wraps the shared layouter.
pub struct Environment {
    layouter: Arc<RwLock<Layouter>>,
}

impl Environment {
    /// Returns a new environment with the given layouter.
    pub fn new(layouter: Arc<RwLock<Layouter>>) -> Environment {
        Environment { layouter }
    }

    /// Returns a reference to the shared layouter.
    pub fn layouter(&mut self) -> &Arc<RwLock<Layouter>> {
        &self.layouter
    }
}
