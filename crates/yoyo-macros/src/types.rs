use polyhorn_ui::styles::Transform;
use strum_macros::EnumString;

#[derive(Copy, Clone, Debug, EnumString)]
pub enum Transition {
    #[strum(serialize = "step")]
    Step,

    #[strum(disabled)]
    EaseInOut(f32),

    #[strum(disabled)]
    Spring(Spring),

    #[strum(disabled)]
    Delay(f32),
}

impl Default for Transition {
    fn default() -> Self {
        Transition::Step
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Spring {
    pub stiffness: f32,
    pub damping: f32,
    pub mass: f32,
    pub allows_overdamping: bool,
    pub overshoot_clamping: bool,
}

impl Default for Spring {
    fn default() -> Self {
        Spring {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            allows_overdamping: false,
            overshoot_clamping: false,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Transitions {
    pub opacity: Transition,
    pub transform: TransformTransition,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct TransformTransition {
    pub translation: Transition,
    pub scale: Transition,
    pub skew: Transition,
    pub perspective: Transition,
    pub rotation: Transition,
}

#[derive(Copy, Clone, Debug)]
pub struct Style {
    pub opacity: f32,
    pub transform: [Transform<f32>; 8],

    pub transitions: Transitions,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            opacity: 1.0,
            transform: Default::default(),
            transitions: Default::default(),
        }
    }
}
