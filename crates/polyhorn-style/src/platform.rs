use std::fmt::Debug;

pub trait Platform {
    type Color: Clone + Debug + Default + Send + Sync;
    type Font: Clone + Debug + Default + Send + Sync;
}
