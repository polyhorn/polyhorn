#[derive(Copy, Clone, Debug)]
pub enum Transition {
    Step,
    EaseInOut,
    Spring(Spring),
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

#[derive(Copy, Clone, Debug)]
pub struct Style {
    pub opacity: f32,
    pub opacity_transition: Transition,
    pub transform_translation_x: f32,
    pub transform_translation_x_transition: Transition,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            opacity: 1.0,
            opacity_transition: Transition::Step,
            transform_translation_x: 0.0,
            transform_translation_x_transition: Transition::Step,
        }
    }
}
