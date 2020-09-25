use super::Layouter;
use std::sync::{Arc, RwLock};

pub struct Environment {
    layouter: Arc<RwLock<Layouter>>,
}

impl Environment {
    pub fn new(layouter: Arc<RwLock<Layouter>>) -> Environment {
        Environment { layouter }
    }

    pub fn layouter(&mut self) -> &Arc<RwLock<Layouter>> {
        &self.layouter
    }
}
