use super::Transition;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Style {
    pub opacity: f32,

    /// The tx component of the transformation matrix. Note: this value is in
    /// pixels.
    pub transform_translation_x: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transitions {
    pub opacity: Transition,

    pub transform_translation_x: Transition,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            opacity: 1.0,
            transform_translation_x: 0.0,
        }
    }
}
