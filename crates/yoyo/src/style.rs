use polyhorn_ui::styles::Transform;

use super::{TransformTransition, Transition};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Style {
    pub opacity: f32,
    pub transform: [Transform<f32>; 8],
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transitions {
    pub opacity: Transition,
    pub transform: TransformTransition,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            opacity: 1.0,
            transform: Default::default(),
        }
    }
}
