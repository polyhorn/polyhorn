use polyhorn_ui::styles::Transform;

use super::Transition;

#[derive(Copy, Clone, Debug)]
pub enum PropertyValue {
    /// Controls the opacity of a view that is used for blending during
    /// composition.
    Opacity(f32),

    /// Controls the transformation matrices that are applied to a view during
    /// composition.
    Transform([Transform<f32>; 8]),

    /// Controls the transition between opacity of different variants.
    TransitionOpacity(Transition),

    /// Controls the transition between transforms of different variants.
    TransitionTransform(Transition),
}
